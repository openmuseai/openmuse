import XCTest

final class OfficeHarnessAcceptanceUITests: XCTestCase {
  private let app = XCUIApplication(bundleIdentifier: "com.openmuseai.mobile")

  override func setUpWithError() throws {
    continueAfterFailure = false
    // Activate rather than launch: preserve the existing document and session.
    app.activate()
    XCTAssertTrue(app.wait(for: .runningForeground, timeout: 20))
    XCTAssertTrue(app.windows.firstMatch.waitForExistence(timeout: 20))
    XCTAssertEqual(app.secureTextFields.count, 0,
                   "Do not inspect or record an authentication screen")
  }

  private func snapshot(_ name: String) {
    let image = XCTAttachment(screenshot: app.screenshot())
    image.name = name
    image.lifetime = .keepAlways
    add(image)
    let hierarchy = XCTAttachment(string: app.debugDescription)
    hierarchy.name = name + "-accessibility"
    hierarchy.lifetime = .keepAlways
    add(hierarchy)
    print("OFFICE_HARNESS_UI_BEGIN\n\(app.debugDescription)\nOFFICE_HARNESS_UI_END")
  }

  func test00ObserveAndPreserveCurrentGuide() throws {
    let guide = app.staticTexts.matching(
      NSPredicate(format: "label CONTAINS %@", "OpenMuse 智能文档使用指南.musedoc")
    ).firstMatch
    // Only commit an existing edit when it is our bundled QA guide.
    if guide.exists {
      let done = app.buttons["完成"]
      if done.exists && done.isHittable {
        done.tap()
      }
    }
    snapshot("01-installed-channel-document")
  }

  func test01ObserveOwnTestFlightUpdate() throws {
    let testFlight = XCUIApplication(bundleIdentifier: "com.apple.TestFlight")
    testFlight.activate()
    XCTAssertTrue(testFlight.wait(for: .runningForeground, timeout: 20))
    XCTAssertEqual(testFlight.secureTextFields.count, 0)
    testFlight.scrollViews["AppList.ScrollView"].swipeDown()
    let ownApp = testFlight.staticTexts["Office Harness"].firstMatch
    XCTAssertTrue(ownApp.waitForExistence(timeout: 20),
                  "Only the existing Office Harness beta may be selected")
    ownApp.tap()
    let detail = testFlight.buttons.matching(NSPredicate(format:
      "label CONTAINS[c] 'Previous Builds' OR label CONTAINS[c] 'Update' OR label CONTAINS[c] 'Stop Testing'"
    )).firstMatch
    _ = detail.waitForExistence(timeout: 15)
    let image = XCTAttachment(screenshot: testFlight.screenshot())
    image.name = "02-own-testflight-detail"
    image.lifetime = .keepAlways
    add(image)
    print("OWN_TESTFLIGHT_UI_BEGIN\n\(testFlight.debugDescription)\nOWN_TESTFLIGHT_UI_END")
  }

  func test02InstallOwn112502FromTestFlight() throws {
    let testFlight = XCUIApplication(bundleIdentifier: "com.apple.TestFlight")
    testFlight.activate()
    XCTAssertTrue(testFlight.wait(for: .runningForeground, timeout: 20))
    XCTAssertEqual(testFlight.secureTextFields.count, 0)
    XCTAssertTrue(testFlight.staticTexts["1.0.3 (112502) · 90 days"].waitForExistence(timeout: 15))
    let ownInstall = testFlight.buttons["AppList.InstallButton.6827192902"]
    XCTAssertTrue(ownInstall.waitForExistence(timeout: 10))
    let update = ownInstall.buttons["Update"].firstMatch
    XCTAssertTrue(update.exists)
    update.tap()
    XCTAssertTrue(ownInstall.buttons["Open"].firstMatch.waitForExistence(timeout: 180),
                  "The channel update must finish before inspecting the App")
    app.activate()
    XCTAssertTrue(app.wait(for: .runningForeground, timeout: 20))
    snapshot("03-after-testflight-112502-update")
  }

  func test03InspectGuideTitleAndEditor112502() throws {
    let guide = app.otherElements.matching(NSPredicate(format:
      "label BEGINSWITH %@", "OpenMuse 智能文档使用指南.musedoc\nDocuments"
    )).firstMatch
    XCTAssertTrue(guide.waitForExistence(timeout: 15))
    guide.tap()
    XCTAssertTrue(app.buttons["编辑"].waitForExistence(timeout: 20))
    snapshot("04-guide-title-112502")
    app.buttons["编辑"].tap()
    XCTAssertTrue(app.buttons["完成"].waitForExistence(timeout: 15))
    snapshot("05-guide-editor-112502-before-qa-edit")
  }

  func test04AppendSaveAndReopenGuide112502() throws {
    let guideTitle = app.otherElements.matching(NSPredicate(format:
      "label == %@", "OpenMuse 智能文档使用指南.musedoc\nDocuments"
    )).firstMatch
    XCTAssertTrue(guideTitle.exists)
    XCTAssertTrue(app.buttons["完成"].exists)
    XCTAssertTrue(app.keyboards.firstMatch.exists)
    let markerText = "qaiphonesave"
    XCTAssertFalse(app.staticTexts.matching(NSPredicate(format:
      "label CONTAINS %@", markerText)).firstMatch.exists)
    // The previous observation shows the caret in the guide's final blank paragraph.
    // Append a QA marker without selecting or replacing existing guide content.
    let blank = app.staticTexts.matching(NSPredicate(format: "label == ' '")).allElementsBoundByIndex
      .last(where: { $0.isHittable })
    XCTAssertNotNil(blank)
    blank!.tap()
    let keyboard = app.keyboards.firstMatch
    let shift = keyboard.buttons["shift"]
    if shift.exists && shift.isSelected { shift.tap() }
    // Flutter's hidden TextInput is not exposed as an XCTest-focused descendant.
    // Use actual visible system keys instead of the focused-element typeText API.
    for character in markerText {
      keyboard.keys.matching(NSPredicate(format: "label ==[c] %@", String(character)))
        .firstMatch.tap()
    }
    keyboard.buttons["Return"].tap()
    let marker = app.staticTexts.matching(NSPredicate(format:
      "label CONTAINS %@", markerText)).firstMatch
    XCTAssertTrue(marker.waitForExistence(timeout: 10))
    snapshot("06-guide-appended-qa-marker")
    app.buttons["完成"].tap()
    XCTAssertTrue(app.buttons["编辑"].waitForExistence(timeout: 15))
    // The save-status title is outside the lazy accessibility viewport at the
    // bottom. Verify persistence by reopening instead of assuming it is exposed.
    try reopenGuideAndAssertMarker(markerText)
  }

  func test05VerifySavedGuide112502() throws {
    XCTAssertTrue(app.buttons["编辑"].waitForExistence(timeout: 15))
    try reopenGuideAndAssertMarker("qaiphonesave")
  }

  func test06InspectBundledPresentation112502() throws {
    XCTAssertTrue(app.buttons["编辑"].exists)
    app.buttons["返回"].tap()
    let sample = app.otherElements.matching(NSPredicate(format:
      "label BEGINSWITH %@", "OpenMuse Introduction.pptx\nDocuments"
    )).firstMatch
    XCTAssertTrue(sample.waitForExistence(timeout: 15))
    sample.tap()
    XCTAssertTrue(app.webViews.firstMatch.waitForExistence(timeout: 30))
    _ = app.staticTexts["OpenMuse"].firstMatch.waitForExistence(timeout: 30)
    snapshot("08-bundled-presentation-112502")
  }

  func test07FocusPresentationSubtitle112502() throws {
    XCTAssertTrue(app.otherElements["OpenMuse Introduction.pptx"].exists)
    XCTAssertTrue(app.staticTexts["幻灯片 1 / 4"].exists)
    // Coordinates come from the observed 428x926 physical-device screenshot:
    // the first slide's subtitle is inside the canvas at (140,525).
    XCTAssertEqual(app.windows.firstMatch.frame.width, 428, accuracy: 1)
    app.coordinate(withNormalizedOffset: .zero)
      .withOffset(CGVector(dx: 140, dy: 525)).doubleTap()
    _ = app.keyboards.firstMatch.waitForExistence(timeout: 10)
    snapshot("09-presentation-subtitle-focus")
  }

  func test08InspectPresentationTouchMenu112502() throws {
    XCTAssertTrue(app.otherElements["OpenMuse Introduction.pptx"].exists)
    app.coordinate(withNormalizedOffset: .zero)
      .withOffset(CGVector(dx: 140, dy: 525)).press(forDuration: 1.2)
    snapshot("10-presentation-touch-context-menu")
  }

  private func reopenGuideAndAssertMarker(_ markerText: String) throws {
    app.buttons["返回"].tap()
    let recent = app.otherElements.matching(NSPredicate(format:
      "label BEGINSWITH %@", "OpenMuse 智能文档使用指南.musedoc\nDocuments"
    )).firstMatch
    XCTAssertTrue(recent.waitForExistence(timeout: 10))
    recent.tap()
    XCTAssertTrue(app.buttons["编辑"].waitForExistence(timeout: 15))
    let marker = app.staticTexts.matching(NSPredicate(format:
      "label CONTAINS %@", markerText)).firstMatch
    // Native document blocks are lazy: the final paragraph is exposed after
    // scrolling it into the viewport, rather than at the initial top position.
    for _ in 0..<18 {
      if marker.exists && marker.isHittable { break }
      app.swipeUp()
    }
    XCTAssertTrue(marker.exists && marker.isHittable,
                  "Read the persisted native document after closing its session")
    let guideTitle = app.otherElements.matching(NSPredicate(format:
      "label == %@", "OpenMuse 智能文档使用指南.musedoc\nDocuments"
    )).firstMatch
    XCTAssertLessThan(guideTitle.frame.minY, 100,
                      "The document name must remain aligned with the top resource header")
    snapshot("07-guide-marker-persisted-after-reopen")
  }
}
