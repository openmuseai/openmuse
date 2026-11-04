Pod::Spec.new do |s|
  s.name = 'openmuse_siyuan_client'
  s.version = '0.1.0'
  s.summary = 'Local SiYuan client kernel, owned by the plugin.'
  s.homepage = 'https://github.com/siyuan-note/siyuan'
  s.license = { :type => 'AGPL-3.0-or-later', :file => '../LICENSE' }
  s.author = { 'OpenMuse contributors' => 'opensource@openmuseai.com' }
  s.source = { :path => '.' }
  s.dependency 'FlutterMacOS'
  s.platform = :osx, '12.0'
  s.vendored_libraries = 'Libraries/libmuse_siyuan.dylib'
  s.resources = 'Libraries/libonnxruntime.dylib'
end
