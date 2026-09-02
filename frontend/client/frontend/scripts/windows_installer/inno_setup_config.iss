[Setup]
AppName=DSH Office
AppVersion={#AppVersion}
AppPublisher=OpenMuseAI
WizardStyle=modern
Compression=lzma2
SolidCompression=yes
DefaultDirName={autopf}\DSH Office\
DefaultGroupName=DSH Office
SetupIconFile=flowy_logo.ico
UninstallDisplayIcon={app}\dsh-office.exe
UninstallDisplayName=DSH Office
VersionInfoVersion={#AppVersion}
UsePreviousAppDir=no

[Files]
Source: "dsh-office\dsh-office.exe"; DestDir: "{app}"; DestName: "dsh-office.exe"; Flags: ignoreversion
Source: "dsh-office\*";DestDir: "{app}"
Source: "dsh-office\data\*";DestDir: "{app}\data\"; Flags: recursesubdirs

[Icons]
Name: "{userdesktop}\DSH Office"; Filename: "{app}\dsh-office.exe"
Name: "{group}\DSH Office"; Filename: "{app}\dsh-office.exe"

[Registry]
Root: HKCR; Subkey: "dsh-office"; ValueType: "string"; ValueData: "URL:Custom Protocol"; Flags: uninsdeletekey
Root: HKCR; Subkey: "dsh-office"; ValueType: "string"; ValueName: "URL Protocol"; ValueData: ""
Root: HKCR; Subkey: "dsh-office\DefaultIcon"; ValueType: "string"; ValueData: "{app}\dsh-office.exe,0"
Root: HKCR; Subkey: "dsh-office\shell\open\command"; ValueType: "string"; ValueData: """{app}\dsh-office.exe"" ""%1"""