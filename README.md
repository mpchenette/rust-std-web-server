# chenette.com

## Notes
Good reference links for some of the conventions and best practices used in this repository can be found below.
- [File Names and variable naming conventions](https://docs.microsoft.com/en-us/azure/cloud-adoption-framework/ready/azure-best-practices/resource-abbreviations)
- [Resource naming convention guidelines](https://docs.microsoft.com/en-us/azure/cloud-adoption-framework/ready/azure-best-practices/resource-naming)

- [OpenID Connect for Github actions in Azure](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/configuring-openid-connect-in-azure)
- [Reference for rough workflow structure](https://github.com/marketplace/actions/azure-webapp#sample-workflow-to-build-and-deploy-a-nodejs-app-to-containerized-webapp-using-azure-service-principal)


idea: keep app code and infra code in separate repos but make sure that when you are running cicd, you are required to run the infra when you deploy the app code. And in the build logs it should be abundantly clear which version of infra code (ie the branch sha or something) and which version of the app code is being used (for trackability and retro problem solving). All repos should be vizible to everyone (to avoid security through obscurity) but we can then scope permissions to those who need them, limiting the potential of bad actors. Also this is much more amenable to code atomicity???? (not sure if that's the right word) so if one team supports the infra and a different team supports the app code, they wont get confused and they are working with far more simple repos. This should also lend itself to easier testing of each (and with the app code, potentiallly making it easier to copy the app code locally and work with tools like docker, without worrying about the infra and it's files). Keep in mind though that one downside is that you will essentially need 2 repos for each app now. If you (mistakenly) go the route of having central repos that any/all apps can use to deploy infra, you are essentially just creating an unessesary middle man between the ARM api and your app (in otherwords, it makes more sense for each app to choose and specify its infra for itself. Or does it? Maybe something like cvx deos, where we write our own "roles" or infra templates that new apps can use makes more sense. This way we are standardizing the way the company deploys apps. So we can allow the teams to specify vars if they need but we give them default values to start with)

mission: to make solutions easy enough to use and prevalent enough and intuitive enough that even the smallest of small businesses (10 or less) can utilize and reap the benefits of them. Ultimate goal is to automate legacy tasks, digitize old papers and processes and make documentation and role handoff/handover seamless and painless for all companies.

## Ideas
learning chess moves / chess training

## To-Do
- Add to CI/CD
    - OWASP ZAP
    - SonarCloud
