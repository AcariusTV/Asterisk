## Documentation - Configurating [Asterisk](https://github.com/AcariusTV/Asterisk)
To properly use Asterisk, you'll need to learn how to interact with the configuration file. On this page, you'll find a step-by-step guide on how to configure Asterisk and what options are available. Let's get started!

### The Asterisk.yml configuration file
For the Asterisk build tool, the configuration language standard is set to [YAML](https://yaml.org/). If you don't know this configuration language at all, I would highly recommend watching a quick tutorial first. If you know what [YAML](https://yaml.org/) is, you can directly start!

Once you open up your Asterisk.yml file, you'll propably see something like this (If you set it up using `ast init`):
```YAML
package:
  name: {yourProgramName}
  version: 0.1.0

dependencies:
  # Add your dependencies here
```
We'll now have a look at every option you can add to this barebones configuration file to make your building processes better:
- `name` - The official name of your package, which will also be used as the name of your executable.
- `version` - The version of your package/build.
- `description` - The description of your package.
- `authors` - The authors who worked on this project.
- `license` - The license you're using (e.g. MIT, GPL 3.0,...).
- `target` - The target platform for which you want to build.
