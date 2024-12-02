set shell := ["pwsh","-NoProfile","-NoLogo","-Command"]

default:
  @just --choose

check-postgres-service:
    #!pwsh
    Get-Service -Name postgresql*

stop-postgres-service
    #!pwsh
    Stop-Service -Name postgresql*

start-postgres-service:
    #!pwsh
    Start-Service -Name postgresql*