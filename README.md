# Hodor
<i>"Hold the door" as he said</i>

## What?
As its name suggest, Hodor is the door to your applications. It is blazingly fast and easy to use Identity Provider written in Rust.

## Why?
Keycloak is leader in its field as open source IDP but it really complicated to configure, especially for non security expert. Hodor aims to reduce the pain and complexity of understanding the mechanisms behind most common authentication processes such as OAuth2 and OpenID Connect by prodiving a solution where there is minimal configuration to make it work in a production grade environment.

## When?
If you have an application that requires authenticating users and you don't want to spend a lot of time understand different ways to do it. You still want to use web standard or need single sign-on.

Here are some example cases when you can use Hodor:
- Single Page application or mobile application without a backend, then use the PKCE flow. See how to configure PKCE.
- Web application with a backend, then use the Authorization Code Flow. See how to configure the Authorization Code Flow.
- When you want to call an API with such as Requestly or Postman with only a user and password, then use the Password Flow. See how to configure the Password Flow.
- When you have an application that needs to call another application, then use the Client Credentials Flow. See how to configure the Client Credentials Flow.
- When you want to use JWT tokens, or have user information in the token, then use the OpenID Connect. See how to configure the OpenID Connect.

## Not when?
Hodor is not an authorization server.

## Concepts
Hodor is primarily an OAuth2 based Identity Provider.

### Client

### User

### Token

## Features

### OAuth2 - Password Flow

### OAuth2 - Client Credentials Flow

### OAuth2 - Authorization Code Flow

### OAuth2 - Implicit Flow
This flow is not supported and it is discouraged to use as it is not secure.

### OpenID Connect
- JWT token
- signature
- validations des claims
- custom claims
- pour l'activer il faut ajouter le scope openid