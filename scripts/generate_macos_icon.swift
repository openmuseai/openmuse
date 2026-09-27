import AppKit
import Foundation

// OpenMuse's first-party macOS icon. Run from the repository root with:
// swift scripts/generate_macos_icon.swift app/openmuse_host/macos/Runner/Assets.xcassets/AppIcon.appiconset
let destination = CommandLine.arguments.count > 1
  ? URL(fileURLWithPath: CommandLine.arguments[1], isDirectory: true)
  : URL(fileURLWithPath: "app/openmuse_host/macos/Runner/Assets.xcassets/AppIcon.appiconset", isDirectory: true)

func sparkle(center: NSPoint, horizontal: CGFloat, vertical: CGFloat, waist: CGFloat) -> NSBezierPath {
  let path = NSBezierPath()
  path.move(to: NSPoint(x: center.x, y: center.y + vertical))
  path.curve(to: NSPoint(x: center.x + horizontal, y: center.y),
             controlPoint1: NSPoint(x: center.x + waist, y: center.y + waist),
             controlPoint2: NSPoint(x: center.x + waist, y: center.y + waist))
  path.curve(to: NSPoint(x: center.x, y: center.y - vertical),
             controlPoint1: NSPoint(x: center.x + waist, y: center.y - waist),
             controlPoint2: NSPoint(x: center.x + waist, y: center.y - waist))
  path.curve(to: NSPoint(x: center.x - horizontal, y: center.y),
             controlPoint1: NSPoint(x: center.x - waist, y: center.y - waist),
             controlPoint2: NSPoint(x: center.x - waist, y: center.y - waist))
  path.curve(to: NSPoint(x: center.x, y: center.y + vertical),
             controlPoint1: NSPoint(x: center.x - waist, y: center.y + waist),
             controlPoint2: NSPoint(x: center.x - waist, y: center.y + waist))
  path.close()
  return path
}

for size in [16, 32, 64, 128, 256, 512, 1024] {
  guard let bitmap = NSBitmapImageRep(
    bitmapDataPlanes: nil, pixelsWide: size, pixelsHigh: size,
    bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true,
    isPlanar: false, colorSpaceName: .deviceRGB,
    bytesPerRow: 0, bitsPerPixel: 0
  ), let context = NSGraphicsContext(bitmapImageRep: bitmap) else {
    fatalError("Cannot make icon bitmap at \(size)px")
  }
  NSGraphicsContext.saveGraphicsState()
  NSGraphicsContext.current = context
  context.imageInterpolation = .high
  context.shouldAntialias = true
  let scale = CGFloat(size) / 1024
  context.cgContext.scaleBy(x: scale, y: scale)
  context.cgContext.clear(CGRect(x: 0, y: 0, width: 1024, height: 1024))

  let tile = NSBezierPath(roundedRect: NSRect(x: 28, y: 28, width: 968, height: 968), xRadius: 218, yRadius: 218)
  NSGradient(starting: NSColor(calibratedRed: 0.28, green: 0.40, blue: 0.80, alpha: 1),
             ending: NSColor(calibratedRed: 0.15, green: 0.25, blue: 0.58, alpha: 1))!
    .draw(in: tile, angle: 135)

  context.cgContext.saveGState()
  tile.addClip()
  NSColor(calibratedWhite: 1, alpha: 0.11).setFill()
  NSBezierPath(ovalIn: NSRect(x: 510, y: 515, width: 545, height: 545)).fill()
  NSColor.white.setFill()
  sparkle(center: NSPoint(x: 472, y: 501), horizontal: 294, vertical: 308, waist: 79).fill()
  NSColor(calibratedRed: 0.72, green: 0.89, blue: 1, alpha: 1).setFill()
  sparkle(center: NSPoint(x: 746, y: 735), horizontal: 97, vertical: 106, waist: 26).fill()
  sparkle(center: NSPoint(x: 702, y: 267), horizontal: 57, vertical: 63, waist: 15).fill()
  context.cgContext.restoreGState()

  context.flushGraphics()
  NSGraphicsContext.restoreGraphicsState()
  guard let data = bitmap.representation(using: .png, properties: [:]) else {
    fatalError("Cannot encode icon at \(size)px")
  }
  try data.write(to: destination.appendingPathComponent("app_icon_\(size).png"))
}
