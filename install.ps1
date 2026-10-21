cargo build --release

$BinDir = "$HOME\.local\bin"
if (!(Test-Path $BinDir)) { New-Item -ItemType Directory -Path $BinDir -Force }

Copy-Item ".\target\release\wall_shuff.exe" -Destination "$BinDir\wall_shuff.exe" -Force
if (Test-Path ".\target\release\wall_shuffd.exe") {
    Copy-Item ".\target\release\wall_shuffd.exe" -Destination "$BinDir\wall_shuffd.exe" -Force
}

$Action = New-ScheduledTaskAction -Execute "$BinDir\wall_shuffd.exe"
$Trigger = New-ScheduledTaskTrigger -AtLogOn
$Settings = New-ScheduledTaskSettingsSet -StartWhenAvailable -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit 0

Register-ScheduledTask -TaskName "Wall_ShuffDaemon" -Action $Action -Trigger $Trigger -Settings $Settings -Force

Start-ScheduledTask -TaskName "Wall_ShuffDaemon"
