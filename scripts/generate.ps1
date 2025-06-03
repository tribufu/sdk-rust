#!/usr/bin/env pwsh

java -jar ./vendor/openapi-generator/openapi-generator-cli.jar generate `
    -i https://api.tribufu.com/openapi.json `
    -g rust `
    -o . `
    --global-property apis,models,supportingFiles,apiDocs=false,modelDocs=false,apiTests=false,modelTests=false `
    --additional-properties=packageName=tribufu,library=reqwest-trait,supportAsync=true,preferUnsignedInt=true `
    --openapi-normalizer SET_TAGS_FOR_ALL_OPERATIONS=Tribufu `
    --skip-validate-spec
