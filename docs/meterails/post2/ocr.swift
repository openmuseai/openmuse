import Foundation
import Vision
import AppKit

let args = CommandLine.arguments
guard args.count > 1 else {
    FileHandle.standardError.write("usage: ocr <image>...\n".data(using: .utf8)!)
    exit(2)
}

for path in args.dropFirst() {
    guard let img = NSImage(contentsOfFile: path),
          let tiff = img.tiffRepresentation,
          let bitmap = NSBitmapImageRep(data: tiff),
          let cg = bitmap.cgImage else {
        print("=== \(path) === ERROR: cannot load image")
        continue
    }
    print("=== \(path) === (\(cg.width)x\(cg.height))")
    let request = VNRecognizeTextRequest()
    request.recognitionLevel = .accurate
    request.recognitionLanguages = ["zh-Hans", "en-US"]
    request.usesLanguageCorrection = true
    let handler = VNImageRequestHandler(cgImage: cg, options: [:])
    do {
        try handler.perform([request])
    } catch {
        print("ERROR: \(error)")
        continue
    }
    guard let observations = request.results else { continue }
    // sort top-to-bottom, left-to-right
    let sorted = observations.sorted { a, b in
        let ay = a.boundingBox.midY, by = b.boundingBox.midY
        if abs(ay - by) > 0.012 { return ay > by }
        return a.boundingBox.minX < b.boundingBox.minX
    }
    for obs in sorted {
        guard let top = obs.topCandidates(1).first else { continue }
        let bb = obs.boundingBox
        let x = String(format: "%.3f", bb.minX)
        let y = String(format: "%.3f", 1 - bb.maxY)
        print("[\(x),\(y)] \(top.string)")
    }
    print("")
}
