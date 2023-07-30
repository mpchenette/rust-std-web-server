// =========== asp.bicep ===========

// USER-PROVIDED PARAMETERS
param application string
param environment string
param instance string = '1'

// SKU PARAMETERS
param skuCapacity int = 1
param skuFamily string = 'B'
param skuName string = 'B1'
param skuSize string = 'B1'
param skuTier string = 'Basic'

// PROPERTIES PARAMETERS
param isLinux bool = true

// BASE PARAMETERS
param name string = 'asp-${application}-${environment}-${location}-${padLeft(instance, 3, '0')}'
param location string = resourceGroup().location
param sku object = {
  capacity: skuCapacity
  family: skuFamily
  name: skuName
  size: skuSize
  tier: skuTier
}
param properties object = {
  reserved: isLinux
}

// RESOURCE
resource asp 'Microsoft.Web/serverfarms@2022-09-01' = {
  name: name
  location: location
  sku: sku
  properties: properties
}

// OUTPUTS
output resourceId string = asp.id
