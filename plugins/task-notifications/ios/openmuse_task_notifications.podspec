Pod::Spec.new do |s|
  s.name = 'openmuse_task_notifications'
  s.version = '0.1.0'
  s.summary = 'Opt-in, content-free local task notifications.'
  s.description = 'Local task notifications using the iOS UserNotifications framework.'
  s.homepage = 'https://openmuseai.com'
  s.license = { :type => 'AGPL-3.0', :file => '../LICENSE' }
  s.author = { 'OpenMuse' => 'developer@openmuseai.com' }
  s.source = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '14.0'
  s.swift_version = '5.0'
  s.frameworks = 'UserNotifications', 'UIKit'
end
