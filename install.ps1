cargo build --release

$BinDir = "$HOME\.local\bin"
if (!(Test-Path $BinDir)) { New-Item -ItemType Directory -Path $BinDir -Force }
Copy-Item ".\target\release\wall_shuff.exe" -Destination "$BinDir\wall_shuff.exe" -Force

$Action = New-ScheduledTaskAction -Execute "$BinDir\wall_shuff.exe"
$Trigger = New-ScheduledTaskTrigger -Daily -At "00:00"
$Settings = New-ScheduledTaskSettingsSet -StartWhenAvailable -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries

Register-ScheduledTask -TaskName "Wall_Shuff" -Action $Action -Trigger $Trigger -Settings $Settings -Force
