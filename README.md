# Athalar

Athalar is a specification for generating configuration. It enables you to write your
configuration in a single format and is especially useful when you want to share
configurations across multiple languages. It transforms your configuration into language
bindings specific to your requirements.

## Example

Lets consider the following mappings:

```yaml
# ./backend.ath.yaml
bindings:
  - typescript:
      output: ./some/path/backend.ts
      adapter: class-validator
      className: ApplicationConfig
variables:
  - !IncludePartial mail
  - EXTERNAL_API_URL:
      validators:
        - is_string
        - is_url
```

```yaml
# ./partials/mail.ath.yaml
MAIL_PORT:
  description: The port at which the mail server is listening at
  validators:
    - is_port
```

Calling the Athalar generators (either through CLI or through language bindings), you will
get the following configuration auto-magically generated:

```typescript
// ./some/path/backend.ts
import { IsString, IsUrl, IsPort } from 'class-validator';

export class ApplicationConfig {
  @IsString()
  @IsUrl()
  EXTERNAL_API_URL: string;

  // The port at which the mail server is listening at
  @IsPort()
  MAIL_PORT: number;
}
```

## Usage

Athalar files are just plain yaml files which are super-charged to provide additional
functionality. It leverages YAML [tags](https://yaml.org/spec/1.2.2/#24-tags) to achieve
this.
