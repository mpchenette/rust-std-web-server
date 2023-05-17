// =========== main.bicep ===========

targetScope = 'subscription'
param application string = 'chenette'
param environment string = 'dev'
param location string = deployment().location

// https://github.com/Azure/bicep/issues/4992
resource rg 'Microsoft.Resources/resourceGroups@2022-09-01' = {
  name: 'rg-${application}-${environment}-${padLeft('1', 3, '0')}'
  location: location
}

module asp '../bicep/resources/asp.bicep' = {
  scope: rg
  name: 'aspDeployment'
  params: {
    application: application
    environment: environment
    location: location
  }
}

module app '../bicep/resources/app.bicep' = {
  scope: rg
  name: 'appDeployment'
  params: {
    application: application
    environment: environment
    location: location
    aspId: asp.outputs.id
  }
}
