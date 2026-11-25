require 'fileutils'
require 'xcodeproj'

# A separate signed UI-test runner targets the already-installed channel App.
# It never builds, replaces or re-signs com.openmuseai.mobile.
destination = File.expand_path(ARGV.fetch(0))
FileUtils.mkdir_p(destination)
source = File.join(__dir__, 'OfficeHarnessAcceptanceUITests.swift')
FileUtils.cp(source, destination)
project = Xcodeproj::Project.new(File.join(destination, 'OfficeHarnessAcceptance.xcodeproj'))
target = project.new_target(:ui_test_bundle, 'OfficeHarnessAcceptanceUITests', :ios, '15.1')
target.source_build_phase.add_file_reference(project.main_group.new_file(File.basename(source)))
project.files.each do |file|
  next unless file.path&.end_with?('Foundation.framework')
  file.path = 'System/Library/Frameworks/Foundation.framework'
  file.source_tree = 'SDKROOT'
end
target.build_configurations.each do |configuration|
  configuration.build_settings.merge!({
    'PRODUCT_BUNDLE_IDENTIFIER' => 'com.openmuseai.qa.release103.uitests',
    'DEVELOPMENT_TEAM' => 'SDPXS943DW',
    'CODE_SIGN_STYLE' => 'Automatic',
    'CODE_SIGN_IDENTITY' => 'Apple Development',
    'GENERATE_INFOPLIST_FILE' => 'YES',
    'SWIFT_VERSION' => '5.0',
    'TARGETED_DEVICE_FAMILY' => '1,2',
    'IPHONEOS_DEPLOYMENT_TARGET' => '15.1',
  })
end
project.save
scheme = Xcodeproj::XCScheme.new
scheme.add_build_target(target)
scheme.add_test_target(target)
scheme.test_action.build_configuration = 'Debug'
scheme.save_as(project.path, 'OfficeHarnessAcceptance', true)
puts project.path
