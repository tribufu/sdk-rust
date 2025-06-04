#!/usr/bin/env pwsh

$crates = @(
    "tribufu-types",
    "tribufu-constants",
    "tribufu-api",
    "tribufu"
)

$batchSize = 6
$waitTime = 300

function Process-Batch {
    param (
        [Parameter(Mandatory=$true)]
        [array]$batch
    )

    foreach ($element in $batch) {
        Write-Output "Processing element $element"
        cargo publish --package $element
    }
}

for ($i = 0; $i -lt $crates.Count; $i += $batchSize) {
    $currentBatch = $crates[$i..($i + $batchSize - 1)]

    Process-Batch -batch $currentBatch

    if ($i -lt ($crates.Count - $batchSize)) {
        Write-Output "Waiting for $waitTime seconds..."
        Start-Sleep -Seconds $waitTime
    }
}
