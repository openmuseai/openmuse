[Setup]
AppName=OpenMuse
AppVersion={#AppVersion}
AppPublisher=OpenMuseAI
WizardStyle=modern
Compression=lzma2
SolidCompression=yes
PrivilegesRequired=lowest
DefaultDirName={localappdata}\Programs\OpenMuse
DefaultGroupName=OpenMuse
UninstallDisplayIcon={app}\OpenMuse.exe
UninstallDisplayName=OpenMuse
VersionInfoVersion={#AppVersion}
UsePreviousAppDir=no
#ifexist "flowy_logo.ico"
SetupIconFile=flowy_logo.ico
#endif

[Files]
Source: "OpenMuse\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{userdesktop}\OpenMuse"; Filename: "{app}\OpenMuse.exe"
Name: "{group}\OpenMuse"; Filename: "{app}\OpenMuse.exe"

[Run]
Filename: "{app}\OpenMuse.exe"; Description: "Launch OpenMuse"; Flags: nowait postinstall skipifsilent

[Registry]
Root: HKCU; Subkey: "Software\Classes\dsh-office"; ValueType: "string"; ValueData: "URL:Custom Protocol"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\dsh-office"; ValueType: "string"; ValueName: "URL Protocol"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\dsh-office\DefaultIcon"; ValueType: "string"; ValueData: "{app}\OpenMuse.exe,0"
Root: HKCU; Subkey: "Software\Classes\dsh-office\shell\open\command"; ValueType: "string"; ValueData: """{app}\OpenMuse.exe"" ""%1"""
Root: HKCU; Subkey: "Software\Classes\openmuse"; ValueType: "string"; ValueData: "URL:OpenMuse"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\openmuse"; ValueType: "string"; ValueName: "URL Protocol"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\openmuse\DefaultIcon"; ValueType: "string"; ValueData: "{app}\OpenMuse.exe,0"
Root: HKCU; Subkey: "Software\Classes\openmuse\shell\open\command"; ValueType: "string"; ValueData: """{app}\OpenMuse.exe"" ""%1"""
