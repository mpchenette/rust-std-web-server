param location string = resourceGroup().location

resource cr 'Microsoft.ContainerRegistry/registries@2023-01-01-preview' = {
  name: 'chenettetestacr'
  location: location
  sku: {
    name: 'Premium'
  }
  properties: {
    adminUserEnabled: true
  }
}

resource kv 'Microsoft.KeyVault/vaults@2023-02-01' = {
  name: 'chenettetestkv'
  location: location
  properties: {
    enabledForTemplateDeployment: true
    tenantId: tenant().tenantId
    accessPolicies: []
    sku: {
      name: 'standard'
      family: 'A'
    }
  }
  resource crUsername 'secrets' = {
    name: 'crUsername'
    properties: {
      value: cr.listCredentials().username
    }
  }
  resource crPassword1 'secrets' = {
    name: 'crPassword1'
    properties: {
      value: cr.listCredentials().passwords[0].value
    }
  }
  resource crPassword2 'secrets' = {
    name: 'crPassword2'
    properties: {
      value: cr.listCredentials().passwords[1].value
    }
  }
}

resource asp 'Microsoft.Web/serverfarms@2022-09-01' = {
  name: 'asp-chenette-prod-southcentralus-001'
  location: location
  sku: {
    capacity: 1
    family: 'B'
    name: 'B1'
    size: 'B1'
    tier: 'Basic'
  }
  properties: {
    reserved: true
  }
}

resource app 'Microsoft.Web/sites@2022-09-01' = {
  name: 'app-chenette-prod-southcentralus-001'
  location: location
  kind: 'app,linux,container'
  properties: {
    httpsOnly: false
    reserved: true
    serverFarmId: asp.id
    siteConfig: {
      acrUseManagedIdentityCreds: true
      alwaysOn: true
      linuxFxVersion: 'DOCKER|crchenetteprod001.azurecr.io/chenette.com:latest'
    }
  }
}

resource acrCredential 'Microsoft.Web/sites/config@2022-09-01' = {
  parent: app
  name: 'appsettings'
  properties: {
    DOCKER_REGISTRY_SERVER_URL: 'https://chenettetestacr.azurecr.io'
    DOCKER_REGISTRY_SERVER_USERNAME: cr.listCredentials().username
    DOCKER_REGISTRY_SERVER_PASSWORD: cr.listCredentials().passwords[0].value
  }
}
