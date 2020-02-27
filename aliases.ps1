Function global:RunCargo64 {cargo run --target x86_64-pc-windows-msvc --release}
Set-Alias -Name cargo_run_x64 -Value RunCargo64 -Scope global
Function global:BuildCargo64 {cargo build --target x86_64-pc-windows-msvc --release}
Set-Alias -Name cargo_build_x64 -Value BuildCargo64 -Scope global
Function global:ExportBinaries{
    Copy-Item .\target\release\mandelbrot.exe -Destination .\binaries\mandelbrot_x86.exe
    Copy-Item .\target\x86_64-pc-windows-msvc\release\mandelbrot.exe -Destination .\binaries\mandelbrot_x64.exe
}
Set-Alias -Name expbin -Value ExportBinaries -Scope global