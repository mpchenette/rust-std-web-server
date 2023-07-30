// =========== app.bicep ===========

// USER-PROVIDED PARAMETERS
param application string
param environment string
param instance string = '1'
param aspId string

// PROPERTIES PARAMETERS
param httpsOnly bool = true
param isLinux bool = true

// BASE PARAMETERS
param name string = 'app-${application}-${environment}-${location}-${padLeft(instance, 3, '0')}'
param location string = resourceGroup().location
param kind string = 'app,linux,container'
param properties object = {
  httpsOnly: httpsOnly
  reserved: isLinux
  serverFarmId: aspId
  siteConfig: {
    alwaysOn: true
    linuxFxVersion: 'DOCKER|crchenetteprod001.azurecr.io/chenette.com:latest'
  }
}

// RESOURCE
resource app 'Microsoft.Web/sites@2022-09-01' = {
  name: name
  location: location
  kind: kind
  properties: properties
}
