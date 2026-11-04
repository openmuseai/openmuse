Pod::Spec.new do |s|
  s.name='openmuse_resource_clipboard'
  s.version='0.1.0'
  s.summary='Typed resource clipboard'
  s.homepage='https://openmuse.io'
  s.license={:type=>'MIT'}
  s.author={'OpenMuse'=>'dev@openmuse.io'}
  s.source={:path=>'.'}
  s.source_files='Classes/**/*'
  s.dependency 'FlutterMacOS'
  s.platform=:osx,'10.15'
  s.swift_version='5.0'
end
