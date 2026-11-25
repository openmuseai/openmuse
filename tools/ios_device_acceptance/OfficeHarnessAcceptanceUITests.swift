import XCTest

final class OfficeHarnessAcceptanceUITests: XCTestCase {
  private let app = XCUIApplication(bundleIdentifier: "com.openmuseai.mobile")

  override func setUpWithError() throws {
    continueAfterFailure = false
    // Activate rather than launch: preserve the existing document and session.
    app.activate()
    XCTAssertTrue(app.wait(for: .runningForeground, timeout: 20))
    XCTAssertTrue(app.windows.firstMatch.waitForExistence(timeout: 20))
    if !name.contains("test12SignInExistingAccount") && !name.contains("test14InspectSignInOutcome") {
      XCTAssertEqual(app.secureTextFields.count, 0,
                     "Do not inspect or record an authentication screen")
    }
  }

  private func snapshot(_ name: String) {
    guard app.secureTextFields.count == 0 else {
      XCTFail("Authentication views must not be captured")
      return
    }
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

  func test09InspectProductionNotificationSettings() throws {
    XCTAssertTrue(app.buttons["返回"].exists)
    app.buttons["返回"].tap()
    XCTAssertTrue(app.staticTexts["我的"].waitForExistence(timeout: 10))
    app.staticTexts["我的"].tap()
    let notifications = app.buttons.matching(NSPredicate(format:
      "label CONTAINS %@", "通知")).firstMatch
    if notifications.waitForExistence(timeout: 5) { notifications.tap() }
    snapshot("11-production-notification-settings")
  }

  func test10InspectExistingAccountLoginControls() throws {
    let signIn = app.staticTexts.matching(NSPredicate(format:
      "label BEGINSWITH %@", "未登录")).firstMatch
    XCTAssertTrue(signIn.exists)
    signIn.tap()
    XCTAssertTrue(app.textFields.firstMatch.waitForExistence(timeout: 10))
    // Do not screenshot authentication views or print field values.
    print("LOGIN_FIELDS " + app.textFields.allElementsBoundByIndex.map(\.label).joined(separator: " | "))
    print("LOGIN_SECURE_FIELDS " + app.secureTextFields.allElementsBoundByIndex.map(\.label).joined(separator: " | "))
    print("LOGIN_BUTTONS " + app.buttons.allElementsBoundByIndex.map(\.label).joined(separator: " | "))
  }

  func test11OpenExistingPasswordSignIn() throws {
    let email = try XCTUnwrap(ProcessInfo.processInfo.environment["MUSE_ACCEPTANCE_EMAIL"])
    let field = app.textFields.firstMatch
    XCTAssertTrue(field.exists)
    field.tap()
    field.typeText(email)
    app.buttons["使用密码继续"].tap()
    XCTAssertTrue(app.secureTextFields.firstMatch.waitForExistence(timeout: 15))
    print("PASSWORD_SIGNIN_BUTTONS " + app.buttons.allElementsBoundByIndex.map(\.label).joined(separator: " | "))
  }

  func test12SignInExistingAccount() throws {
    let password = try XCTUnwrap(ProcessInfo.processInfo.environment["MUSE_ACCEPTANCE_PASSWORD"])
    XCTAssertFalse(password.isEmpty)
    let field = app.secureTextFields.firstMatch
    XCTAssertTrue(field.waitForExistence(timeout: 10))
    field.tap()
    field.typeText(password)
    app.buttons["继续"].tap()
    let signedIn = NSPredicate { _, _ in
      self.app.staticTexts["任务"].exists || self.app.staticTexts["绑定 Desktop"].exists
    }
    XCTAssertEqual(XCTWaiter.wait(for: [XCTNSPredicateExpectation(predicate: signedIn, object: nil)], timeout: 30), .completed)
    XCTAssertFalse(app.secureTextFields.firstMatch.exists)
    if app.staticTexts["任务"].exists { app.staticTexts["任务"].tap() }
    snapshot("12-signed-in-production-task-list")
  }

  func test14InspectSignInOutcome() throws {
    print("SIGNIN_STATUS " + app.staticTexts.allElementsBoundByIndex.map(\.label).joined(separator: " | "))
    print("SIGNIN_BUTTONS " + app.buttons.allElementsBoundByIndex.map(\.label).joined(separator: " | "))
  }

  func test15InspectOnlineDesktopBinding() throws {
    XCTAssertTrue(app.staticTexts["绑定 Desktop"].exists)
    snapshot("13-online-desktop-binding")
  }

  func test16SelectSameAccountOnlineDesktop() throws {
    XCTAssertTrue(app.staticTexts["绑定 Desktop"].exists)
    XCTAssertTrue(app.staticTexts["AlexixideMacBook-Pro.local"].firstMatch.exists)
    // These are already-authorized same-account Desktop connections. This
    // selects a connection; it does not grant a different account access.
    app.buttons["绑定"].firstMatch.tap()
    _ = app.buttons["刷新"].waitForExistence(timeout: 10)
    snapshot("14-same-account-desktop-selected")
  }

  func test17InspectBoundTaskHome() throws {
    if app.staticTexts["绑定 Desktop"].exists {
      XCTAssertTrue(app.staticTexts["已连接"].exists)
      let barrier = app.staticTexts["纱罩"]
      XCTAssertTrue(barrier.exists)
      barrier.tap()
    }
    snapshot("15-bound-production-task-home")
  }

  func test18InspectTaskSessionState() throws {
    snapshot("16-production-task-session-state")
  }

  func test19OpenProductionNotificationSettings() throws {
    XCTAssertTrue(app.staticTexts["我的"].exists)
    app.staticTexts["我的"].tap()
    let row = app.staticTexts.matching(NSPredicate(format:
      "label BEGINSWITH %@", "通知\n")).firstMatch
    XCTAssertTrue(row.waitForExistence(timeout: 10))
    row.tap()
    snapshot("17-production-notification-preference")
  }

  func test20InspectNotificationPermissionRationale() throws {
    let preference = app.switches.firstMatch
    XCTAssertTrue(preference.exists)
    XCTAssertEqual(preference.value as? String, "0")
    preference.tap()
    snapshot("18-notification-permission-rationale")
    let system = XCUIApplication(bundleIdentifier: "com.apple.springboard")
    for alert in system.alerts.allElementsBoundByIndex {
      guard alert.label.contains("Office Harness") else {
        XCTFail("Unexpected system alert; leave it for the user")
        return
      }
      print("NOTIFICATION_ALERT " + alert.label)
      print("NOTIFICATION_ALERT_BUTTONS " + alert.buttons.allElementsBoundByIndex.map(\.label).joined(separator: " | "))
    }
  }

  func test21CancelNotificationRationale() throws {
    XCTAssertTrue(app.buttons["取消"].exists)
    app.buttons["取消"].tap()
    XCTAssertEqual(app.switches.firstMatch.value as? String, "0")
    XCTAssertTrue(app.staticTexts["系统通知\n尚未开启"].exists)
    XCTAssertEqual(XCUIApplication(bundleIdentifier: "com.apple.springboard").alerts.count, 0)
    snapshot("19-cancelled-before-system-permission")
  }

  func test22InspectSystemNotificationPrompt() throws {
    XCTAssertEqual(app.switches.firstMatch.value as? String, "0")
    app.switches.firstMatch.tap()
    app.buttons["继续"].tap()
    let system = XCUIApplication(bundleIdentifier: "com.apple.springboard")
    let alert = system.alerts.firstMatch
    XCTAssertTrue(alert.waitForExistence(timeout: 15))
    XCTAssertTrue(alert.label.contains("Office Harness"))
    print("NOTIFICATION_ALERT " + alert.label)
    print("NOTIFICATION_ALERT_BUTTONS " + alert.buttons.allElementsBoundByIndex.map(\.label).joined(separator: " | "))
    let image = XCTAttachment(screenshot: XCUIScreen.main.screenshot())
    image.name = "20-production-system-notification-prompt"
    image.lifetime = .keepAlways
    add(image)
  }

  func test23DenySystemNotificationPermission() throws {
    let alert = XCUIApplication(bundleIdentifier: "com.apple.springboard").alerts.firstMatch
    XCTAssertTrue(alert.exists)
    XCTAssertTrue(alert.label.contains("Office Harness"))
    alert.buttons["Don’t Allow"].tap()
    XCTAssertTrue(app.switches.firstMatch.waitForExistence(timeout: 10))
    XCTAssertFalse(XCUIApplication(bundleIdentifier: "com.apple.springboard").alerts.firstMatch.exists)
    snapshot("21-production-notification-permission-denied")
  }

  func test24OpenOwnNotificationSystemSettings() throws {
    XCTAssertTrue(app.buttons["前往系统设置"].exists)
    XCTAssertTrue(app.staticTexts["系统通知\n通知已关闭，可在系统设置中开启。"].exists)
    app.buttons["前往系统设置"].tap()
    let settings = XCUIApplication(bundleIdentifier: "com.apple.Preferences")
    XCTAssertTrue(settings.wait(for: .runningForeground, timeout: 15))
    XCTAssertEqual(settings.secureTextFields.count, 0)
    print("OWN_NOTIFICATION_SETTINGS_BEGIN\n\(settings.debugDescription)\nOWN_NOTIFICATION_SETTINGS_END")
    let image = XCTAttachment(screenshot: settings.screenshot())
    image.name = "22-own-notification-system-settings"
    image.lifetime = .keepAlways
    add(image)
  }

  func test25RestoreOwnNotificationPermissionInSystemSettings() throws {
    XCTAssertTrue(app.buttons["前往系统设置"].exists)
    app.buttons["前往系统设置"].tap()
    let settings = XCUIApplication(bundleIdentifier: "com.apple.Preferences")
    XCTAssertTrue(settings.wait(for: .runningForeground, timeout: 15))
    XCTAssertTrue(settings.navigationBars["Office Harness"].exists,
                  "Change only the tested App's notification permission")
    let permission = settings.switches["ALLOW_NOTIFICATIONS_ID"].firstMatch
    XCTAssertTrue(permission.exists)
    XCTAssertEqual(permission.value as? String, "0")
    // iOS exposes an outer row switch and a nested visual switch. Tap the
    // observed visual control, then wait for the asynchronous Settings update.
    permission.coordinate(withNormalizedOffset: CGVector(dx: 0.92, dy: 0.5)).tap()
    let enabled = XCTNSPredicateExpectation(
      predicate: NSPredicate(format: "value == %@", "1"), object: permission)
    XCTAssertEqual(XCTWaiter.wait(for: [enabled], timeout: 10), .completed)
    app.activate()
    XCTAssertTrue(app.wait(for: .runningForeground, timeout: 15))
    snapshot("23-returned-after-notification-permission-restored")
  }

  func test26VerifyRestoredPermissionAndInspectTasks() throws {
    XCTAssertTrue(app.staticTexts["系统通知\n已允许"].waitForExistence(timeout: 15))
    XCTAssertEqual(app.switches.firstMatch.value as? String, "1")
    XCTAssertFalse(app.buttons["前往系统设置"].exists)
    snapshot("24-production-notification-permission-allowed")
    app.buttons["返回"].tap()
    XCTAssertTrue(app.staticTexts["任务"].waitForExistence(timeout: 10))
    app.staticTexts["任务"].tap()
    snapshot("25-production-task-list-for-real-notification-test")
  }

  func test27InspectOwnLatestTestFlightBuild() throws {
    let testFlight = XCUIApplication(bundleIdentifier: "com.apple.TestFlight")
    testFlight.activate()
    XCTAssertTrue(testFlight.wait(for: .runningForeground, timeout: 20))
    XCTAssertEqual(testFlight.secureTextFields.count, 0)
    let list = testFlight.scrollViews["AppList.ScrollView"]
    XCTAssertTrue(testFlight.staticTexts["Office Harness"].firstMatch.waitForExistence(timeout: 30),
                  "Inspect channel content after the initial loading animation")
    if list.exists { list.swipeDown() }
    print("OWN_TESTFLIGHT_UI_BEGIN\n\(testFlight.debugDescription)\nOWN_TESTFLIGHT_UI_END")
    let image = XCTAttachment(screenshot: testFlight.screenshot())
    image.name = "26-own-latest-testflight-channel-build"
    image.lifetime = .keepAlways
    add(image)
    app.activate()
  }

  func test28InstallOwn112503FromTestFlight() throws {
    let testFlight = XCUIApplication(bundleIdentifier: "com.apple.TestFlight")
    testFlight.activate()
    XCTAssertTrue(testFlight.wait(for: .runningForeground, timeout: 20))
    XCTAssertEqual(testFlight.secureTextFields.count, 0)
    let build = testFlight.staticTexts.matching(NSPredicate(
      format: "label BEGINSWITH %@", "1.0.3 (112503)" )).firstMatch
    XCTAssertTrue(build.waitForExistence(timeout: 30))
    let ownInstall = testFlight.buttons["AppList.InstallButton.6827192902"]
    XCTAssertTrue(ownInstall.exists)
    let update = ownInstall.buttons["Update"].firstMatch
    XCTAssertTrue(update.exists)
    update.tap()
    XCTAssertTrue(ownInstall.buttons["Open"].firstMatch.waitForExistence(timeout: 180),
                  "Verify the selected production channel update completes")
    app.activate()
    XCTAssertTrue(app.wait(for: .runningForeground, timeout: 20))
    snapshot("27-after-testflight-112503-update")
  }

  func test29Inspect112503HomeAndPreservedNotificationPermission() throws {
    XCTAssertTrue(app.staticTexts["我的"].waitForExistence(timeout: 30))
    app.staticTexts["我的"].tap()
    XCTAssertFalse(app.staticTexts.matching(NSPredicate(format:
      "label BEGINSWITH %@", "未登录")).firstMatch.exists)
    let row = app.staticTexts.matching(NSPredicate(format:
      "label BEGINSWITH %@", "通知\n")).firstMatch
    XCTAssertTrue(row.waitForExistence(timeout: 10))
    row.tap()
    XCTAssertTrue(app.staticTexts["系统通知\n已允许"].waitForExistence(timeout: 15))
    XCTAssertEqual(app.switches.firstMatch.value as? String, "1")
    snapshot("28-channel-upgrade-preserved-notification-state")
    app.buttons["返回"].tap()
    app.staticTexts["首页"].tap()
    snapshot("29-production-112503-home")
  }

  func test30OpenProduction112503BundledPresentation() throws {
    let sample = app.otherElements.matching(NSPredicate(format:
      "label BEGINSWITH %@", "OpenMuse Introduction.pptx\nDocuments"
    )).firstMatch
    XCTAssertTrue(sample.waitForExistence(timeout: 15))
    sample.tap()
    XCTAssertTrue(app.webViews.firstMatch.waitForExistence(timeout: 30))
    XCTAssertTrue(app.staticTexts["幻灯片 1 / 4"].waitForExistence(timeout: 30))
    snapshot("30-production-112503-presentation-loaded")
  }

  func test31Select112503PresentationSubtitle() throws {
    XCTAssertTrue(app.staticTexts["幻灯片 1 / 4"].exists)
    XCTAssertEqual(app.windows.firstMatch.frame.width, 428, accuracy: 1)
    // Coordinates verified against the actual 112503 channel screenshot.
    app.coordinate(withNormalizedOffset: .zero)
      .withOffset(CGVector(dx: 140, dy: 525)).tap()
    XCTAssertTrue(app.buttons["加粗"].waitForExistence(timeout: 10))
    snapshot("31-production-112503-subtitle-selected")
  }

  func test32Focus112503SelectedPresentationSubtitle() throws {
    XCTAssertTrue(app.buttons["加粗"].exists)
    // Run after test31 as a separate invocation: a second touch on an already
    // selected text box must focus the original editor and OS keyboard.
    app.coordinate(withNormalizedOffset: .zero)
      .withOffset(CGVector(dx: 140, dy: 525)).tap()
    XCTAssertTrue(app.keyboards.firstMatch.waitForExistence(timeout: 15))
    snapshot("32-production-112503-text-editor-and-system-keyboard")
  }

  func test33Type112503PresentationMarker() throws {
    let editor = app.textViews.matching(NSPredicate(format:
      "value CONTAINS %@", "Documents on your phone")).firstMatch
    XCTAssertTrue(editor.exists)
    XCTAssertTrue(app.keyboards.firstMatch.exists)
    // Insert only into the bundled QA subtitle. Never select or replace all.
    editor.typeText(" qa103iospptx")
    XCTAssertTrue((editor.value as? String)?.contains("qa103iospptx") == true)
    snapshot("33-production-112503-real-subtitle-input")
  }

  func test34Commit112503PresentationTextAndObserve() throws {
    let editor = app.textViews.matching(NSPredicate(format:
      "value CONTAINS %@", "qa103iospptx")).firstMatch
    XCTAssertTrue(editor.exists)
    XCTAssertTrue(app.buttons["Done"].exists)
    app.buttons["Done"].tap()
    let fullHeight = XCTNSPredicateExpectation(predicate: NSPredicate { _, _ in
      !self.app.keyboards.firstMatch.exists && self.app.webViews.firstMatch.frame.height > 700
    }, object: nil)
    XCTAssertEqual(XCTWaiter.wait(for: [fullHeight], timeout: 15), .completed)
    snapshot("34-production-112503-subtitle-committed")
  }

  func test35Verify112503PresentationUndoAndRedo() throws {
    XCTAssertFalse(app.keyboards.firstMatch.exists)
    app.buttons["撤销"].tap()
    app.coordinate(withNormalizedOffset: .zero)
      .withOffset(CGVector(dx: 140, dy: 525)).tap()
    let editor = app.textViews.matching(NSPredicate(format:
      "value CONTAINS %@", "Documents on your phone")).firstMatch
    XCTAssertTrue(editor.waitForExistence(timeout: 15))
    XCTAssertFalse((editor.value as? String)?.contains("qa103iospptx") == true)
    snapshot("35-production-112503-undo-removes-marker")
    app.buttons["Done"].tap()
    let fullHeight = NSPredicate { _, _ in
      !self.app.keyboards.firstMatch.exists && self.app.webViews.firstMatch.frame.height > 700
    }
    XCTAssertEqual(XCTWaiter.wait(for: [XCTNSPredicateExpectation(predicate: fullHeight, object: nil)], timeout: 15), .completed)
    app.buttons["恢复"].tap()
    app.coordinate(withNormalizedOffset: .zero)
      .withOffset(CGVector(dx: 140, dy: 525)).tap()
    XCTAssertTrue(editor.waitForExistence(timeout: 15))
    XCTAssertTrue((editor.value as? String)?.contains("qa103iospptx") == true)
    snapshot("36-production-112503-redo-restores-marker")
    app.buttons["Done"].tap()
    XCTAssertEqual(XCTWaiter.wait(for: [XCTNSPredicateExpectation(predicate: fullHeight, object: nil)], timeout: 15), .completed)
  }

  func test36Inspect112503UndoFailure() throws {
    let editor = app.textViews.matching(NSPredicate(format:
      "value CONTAINS %@", "Documents on your phone")).firstMatch
    XCTAssertTrue(editor.exists)
    print("QA_PPTX_SUBTITLE_AFTER_FIRST_UNDO " + (editor.value as? String ?? "<missing>"))
    snapshot("37-production-112503-undo-failure-observation")
    app.buttons["Done"].tap()
    let fullHeight = NSPredicate { _, _ in
      !self.app.keyboards.firstMatch.exists && self.app.webViews.firstMatch.frame.height > 700
    }
    XCTAssertEqual(XCTWaiter.wait(for: [XCTNSPredicateExpectation(predicate: fullHeight, object: nil)], timeout: 15), .completed)
    app.buttons["撤销"].tap()
    app.coordinate(withNormalizedOffset: .zero)
      .withOffset(CGVector(dx: 140, dy: 525)).tap()
    XCTAssertTrue(editor.waitForExistence(timeout: 15))
    print("QA_PPTX_SUBTITLE_AFTER_SECOND_UNDO " + (editor.value as? String ?? "<missing>"))
    snapshot("38-production-112503-second-undo-observation")
  }

  func test37Save112503PresentationAndClose() throws {
    let editor = app.textViews.matching(NSPredicate(format:
      "value CONTAINS %@", "qa103iospptx")).firstMatch
    XCTAssertTrue(editor.exists)
    app.buttons["Done"].tap()
    let save = app.buttons["保存 (Ctrl+S)"]
    XCTAssertTrue(save.waitForExistence(timeout: 10))
    XCTAssertTrue(save.isEnabled)
    save.tap()
    let saved = XCTNSPredicateExpectation(predicate: NSPredicate { _, _ in
      save.exists && !save.isEnabled
    }, object: nil)
    XCTAssertEqual(XCTWaiter.wait(for: [saved], timeout: 20), .completed)
    snapshot("39-production-112503-presentation-saved")
    app.buttons["返回"].tap()
    XCTAssertTrue(app.staticTexts["首页"].waitForExistence(timeout: 15))
    snapshot("40-production-112503-returned-after-save")
  }

  func test38Inspect112503SaveOutcome() throws {
    snapshot("41-production-112503-save-outcome-observation")
  }

  func test39InspectSecondConsecutiveSaveTouch() throws {
    let save = app.buttons["保存 (Ctrl+S)"]
    XCTAssertTrue(save.exists && save.isEnabled)
    // The previous touch left this icon visually hovered. Inspect whether a
    // second uninterrupted touch activates it on physical WebKit.
    save.tap()
    let saved = XCTNSPredicateExpectation(predicate: NSPredicate { _, _ in
      save.exists && !save.isEnabled
    }, object: nil)
    let result = XCTWaiter.wait(for: [saved], timeout: 20)
    snapshot("42-production-112503-second-save-touch-outcome")
    XCTAssertEqual(result, .completed)
  }

  func test40Inspect112503ResourceCloseBarrier() throws {
    XCTAssertTrue(app.otherElements["OpenMuse Introduction.pptx"].exists)
    app.buttons["返回"].tap()
    snapshot("43-production-112503-resource-close-barrier")
  }

  func test41Reopen112503PresentationAndInspectSavedText() throws {
    let sample = app.otherElements.matching(NSPredicate(format:
      "label BEGINSWITH %@", "OpenMuse Introduction.pptx\nDocuments"
    )).firstMatch
    XCTAssertTrue(sample.waitForExistence(timeout: 10))
    sample.tap()
    XCTAssertTrue(app.staticTexts["幻灯片 1 / 4"].waitForExistence(timeout: 30))
    snapshot("44-production-112503-presentation-reopened")
    let point = app.coordinate(withNormalizedOffset: .zero)
      .withOffset(CGVector(dx: 140, dy: 525))
    point.tap()
    XCTAssertTrue(app.buttons["加粗"].waitForExistence(timeout: 10))
    RunLoop.current.run(until: Date(timeIntervalSinceNow: 0.7))
    point.tap()
    let editor = app.textViews.matching(NSPredicate(format:
      "value CONTAINS %@", "Documents on your phone")).firstMatch
    XCTAssertTrue(editor.waitForExistence(timeout: 15))
    snapshot("45-production-112503-reopened-subtitle-value")
    print("QA_PPTX_REOPENED_SUBTITLE " + (editor.value as? String ?? "<missing>"))
    XCTAssertTrue((editor.value as? String)?.contains("qa103iospptx") == true)
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
