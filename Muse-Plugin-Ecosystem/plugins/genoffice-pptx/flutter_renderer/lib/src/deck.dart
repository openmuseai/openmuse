import 'dart:convert';
import 'dart:typed_data';

import 'package:archive/archive.dart';
import 'package:flutter/material.dart';
import 'package:xml/xml.dart';

const pptxMediaType =
    'application/vnd.openxmlformats-officedocument.presentationml.presentation';
const _maxCompressedBytes = 64 * 1024 * 1024;
const _maxExpandedBytes = 192 * 1024 * 1024;

final class PptxDeck {
  PptxDeck({required this.width, required this.height, required this.slides});

  final double width;
  final double height;
  final List<PptxSlide> slides;

  static PptxDeck fromBytes(List<int> bytes) {
    if (bytes.isEmpty || bytes.length > _maxCompressedBytes) {
      throw const FormatException('PPTX exceeds the 64 MiB limit');
    }
    final archive = ZipDecoder().decodeBytes(bytes, verify: true);
    if (archive.length > 4096) throw const FormatException('Too many PPTX parts');
    final parts = <String, List<int>>{};
    var expanded = 0;
    for (final file in archive) {
      if (!file.isFile) continue;
      expanded += file.size;
      if (expanded > _maxExpandedBytes) {
        throw const FormatException('PPTX expanded size exceeds limit');
      }
      if (file.name.startsWith('/') || file.name.split('/').contains('..')) {
        throw const FormatException('Unsafe PPTX part name');
      }
      final dynamic versionedFile = file;
      List<int> content;
      try {
        content = (versionedFile.readBytes() as List<int>?) ?? const [];
      } on NoSuchMethodError {
        content = versionedFile.content as List<int>;
      }
      parts[file.name] = content;
    }
    XmlDocument part(String name) {
      final data = parts[name];
      if (data == null) throw FormatException('Missing PPTX part: $name');
      return XmlDocument.parse(utf8.decode(data));
    }

    final presentation = part('ppt/presentation.xml');
    final size = _first(presentation, 'sldSz');
    final width = _double(size?.getAttribute('cx'), 12192000) / 12700;
    final height = _double(size?.getAttribute('cy'), 6858000) / 12700;
    final ids = _all(presentation, 'sldId')
        .map((element) => element.attributes
            .where((attribute) => attribute.name.local == 'id' &&
                attribute.name.prefix == 'r')
            .map((attribute) => attribute.value)
            .firstOrNull)
        .whereType<String>()
        .toList();
    final relationships = _relationships(part('ppt/_rels/presentation.xml.rels'));
    final slides = <PptxSlide>[];
    for (final id in ids) {
      final target = relationships[id];
      if (target == null) continue;
      final path = _resolve('ppt/presentation.xml', target);
      final xml = part(path);
      final relPath = '${path.substring(0, path.lastIndexOf('/'))}/_rels/${path.split('/').last}.rels';
      final slideRels = parts.containsKey(relPath)
          ? _relationships(part(relPath))
          : <String, String>{};
      final layoutTarget = slideRels.values.where((value) =>
          value.contains('slideLayouts/')).firstOrNull;
      final layout = layoutTarget == null ? null :
          part(_resolve(path, layoutTarget));
      final placeholderBounds = <String, Rect>{};
      if (layout != null) {
        for (final placeholder in _all(layout, 'sp').followedBy(_all(layout, 'pic'))) {
          final marker = _first(placeholder, 'ph');
          final bounds = _bounds(placeholder);
          if (marker == null || bounds == null) continue;
          if (_attr(marker, 'idx') case final String index) {
            placeholderBounds['idx:$index'] = bounds;
          }
          if (_attr(marker, 'type') case final String type) {
            placeholderBounds['type:$type'] = bounds;
          }
        }
      }
      final shapes = <PptxShape>[];
      for (final node in _first(xml, 'spTree')?.children.whereType<XmlElement>() ?? <XmlElement>[]) {
        _readShape(node, path, slideRels, parts, placeholderBounds, shapes);
      }
      // PowerPoint sometimes stores a full-slide picture placeholder after
      // title shapes. It is visually the background; painting XML order would
      // hide the title and every foreground shape.
      bool isBackground(PptxShape shape) => shape.image != null &&
          shape.bounds.width >= width * .9 &&
          shape.bounds.height >= height * .9;
      slides.add(PptxSlide([
        ...shapes.where(isBackground),
        ...shapes.where((shape) => !isBackground(shape)),
      ]));
    }
    if (slides.isEmpty) throw const FormatException('PPTX has no slides');
    return PptxDeck(width: width, height: height, slides: slides);
  }
}

final class PptxSlide {
  PptxSlide(this.shapes);
  final List<PptxShape> shapes;
}

final class PptxShape {
  PptxShape({
    required this.bounds,
    this.text = '',
    this.fill,
    this.stroke,
    this.textColor,
    this.fontSize = 18,
    this.image,
  });
  final Rect bounds;
  final String text;
  final Color? fill;
  final Color? stroke;
  final Color? textColor;
  final double fontSize;
  final Uint8List? image;
}

Iterable<XmlElement> _all(XmlNode root, String local) =>
    root.descendants.whereType<XmlElement>().where((e) => e.name.local == local);

XmlElement? _first(XmlNode root, String local) {
  for (final element in _all(root, local)) {
    return element;
  }
  return null;
}

String? _attr(XmlElement node, String local) {
  for (final attribute in node.attributes) {
    if (attribute.name.local == local) return attribute.value;
  }
  return null;
}

double _double(String? value, double fallback) => double.tryParse(value ?? '') ?? fallback;

Map<String, String> _relationships(XmlDocument xml) => {
  for (final item in _all(xml, 'Relationship'))
    if (item.getAttribute('Id') case final String id)
      if (item.getAttribute('Target') case final String target) id: target,
};

String _resolve(String from, String target) {
  if (target.startsWith('/')) return target.substring(1);
  final segments = [...from.split('/')..removeLast(), ...target.split('/')];
  final result = <String>[];
  for (final segment in segments) {
    if (segment == '..') {
      if (result.isEmpty) throw const FormatException('Invalid PPTX relationship');
      result.removeLast();
    } else if (segment.isNotEmpty && segment != '.') {
      result.add(segment);
    }
  }
  return result.join('/');
}

Color? _color(XmlNode node) {
  final hex = _first(node, 'srgbClr')?.getAttribute('val');
  if (hex != null && RegExp(r'^[0-9A-Fa-f]{6}$').hasMatch(hex)) {
    return Color(0xFF000000 | int.parse(hex, radix: 16));
  }
  return null;
}

void _readShape(
  XmlElement node,
  String slidePath,
  Map<String, String> relationships,
  Map<String, List<int>> parts,
  Map<String, Rect> placeholderBounds,
  List<PptxShape> output,
) {
  if (node.name.local == 'grpSp') {
    for (final child in node.children.whereType<XmlElement>()) {
      _readShape(child, slidePath, relationships, parts, placeholderBounds, output);
    }
    return;
  }
  if (!{'sp', 'pic', 'cxnSp'}.contains(node.name.local)) return;
  final marker = _first(node, 'ph');
  final bounds = _bounds(node) ??
      (marker == null ? null :
        placeholderBounds['idx:${_attr(marker, 'idx')}'] ??
        placeholderBounds['type:${_attr(marker, 'type')}']);
  if (bounds == null) return;
  if (bounds.width <= 0 || bounds.height <= 0) return;
  final paragraphs = _all(node, 'p').map((p) =>
      _all(p, 't').map((t) => t.innerText).join()).where((s) => s.isNotEmpty);
  final text = paragraphs.join('\n');
  Uint8List? image;
  if (node.name.local == 'pic') {
    final embed = _first(node, 'blip');
    final target = embed == null ? null : relationships[_attr(embed, 'embed')];
    if (target != null && !target.contains('://')) {
      final media = parts[_resolve(slidePath, target)];
      if (media != null && media.isNotEmpty) image = Uint8List.fromList(media);
    }
  }
  final shapeStyle = _first(node, 'spPr');
  final fontStyle = _first(node, 'rPr') ?? _first(node, 'defRPr');
  final fontSize = _double(fontStyle?.getAttribute('sz'), 1800) / 100;
  output.add(PptxShape(
    bounds: bounds,
    text: text,
    fill: shapeStyle == null ? null : _color(_first(shapeStyle, 'solidFill') ?? shapeStyle),
    stroke: shapeStyle == null ? null : _color(_first(shapeStyle, 'ln') ?? shapeStyle),
    textColor: fontStyle == null ? null : _color(fontStyle),
    fontSize: fontSize.clamp(7, 72),
    image: image,
  ));
}

Rect? _bounds(XmlElement node) {
  final xfrm = _first(node, 'xfrm');
  final off = xfrm == null ? null : _first(xfrm, 'off');
  final ext = xfrm == null ? null : _first(xfrm, 'ext');
  if (off == null || ext == null) return null;
  return Rect.fromLTWH(
    _double(off.getAttribute('x'), 0) / 12700,
    _double(off.getAttribute('y'), 0) / 12700,
    _double(ext.getAttribute('cx'), 0) / 12700,
    _double(ext.getAttribute('cy'), 0) / 12700,
  );
}
