Get-ChildItem -Path examples -Recurse -Filter *.nmb | ForEach-Object -Parallel {
    Write-Host "`n===== Running: $($_.Name) =====`n"
    ./target/release/nimble.exe run $_.FullName
    Write-Host ""
}
