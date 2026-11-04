Pod::Spec.new do |s|
  s.name = 'openmuse_siyuan_client'
  s.version = '0.1.0'
  s.summary = 'Local SiYuan client kernel, owned by the plugin.'
  s.homepage = 'https://github.com/siyuan-note/siyuan'
  s.license = { :type => 'AGPL-3.0-or-later', :file => '../LICENSE' }
  s.author = { 'OpenMuse contributors' => 'opensource@openmuseai.com' }
  s.source = { :path => '.' }
  s.dependency 'Flutter'
  s.dependency 'onnxruntime-c', '1.24.3'
  s.platform = :ios, '15.1'
  s.static_framework = true
  s.source_files = 'Classes/**/*'
  s.vendored_frameworks = 'Libraries/MuseSiYuan.xcframework'
  s.frameworks = 'CoreFoundation', 'Security', 'SystemConfiguration', 'CoreServices'
  s.libraries = 'resolv', 'z', 'c++'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386 x86_64' }
  s.user_target_xcconfig = {
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386 x86_64',
    # Dart resolves the static C ABI with dlsym. Archive's default "all"
    # stripping removes these globals despite -u retaining their object code.
    'STRIP_STYLE' => 'non-global',
    'OTHER_LDFLAGS' => '$(inherited) -Wl,-u,_muse_siyuan_start -Wl,-u,_muse_siyuan_status -Wl,-u,_muse_siyuan_free -Wl,-u,_muse_siyuan_flush -Wl,-u,_muse_siyuan_suspend -Wl,-u,_muse_siyuan_resume'
  }
end
