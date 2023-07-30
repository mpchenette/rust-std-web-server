// =========== main.bicep ===========
param environment string = 'prod'
param location string = resourceGroup().location

module asp 'asp.bicep' = {
  name: 'chenette.com-asp'
  params: {
    application: 'chenette'
    environment: environment
    location: location
  }
}

module app 'app.bicep' = {
  name: 'chenette.com-app'
  params: {
    application: 'chenette'
    aspId: asp.outputs.resourceId
    environment: environment
    location: location
    httpsOnly: false
  }
}
