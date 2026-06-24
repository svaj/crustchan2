# Project Brief: Crustchan v2

## Overview
Crustchan v2 is a cloud-native, 4chan-style imageboard whose backend is written in Rust. It is a successor to the unfinished [crustchan](https://github.com/devhax-heavy-industry/crustchan) and is designed to be modern, scalable, and infrastructure-agnostic.

## Core Requirements

1. **Imageboard Functionality**
   - Boards with threads and posts
   - Image uploads with optimization (WebP conversion, metadata stripping)
   - Anonymous posting support
   - Optional registered-user boards via OAuth2/OIDC

2. **Modern Frontend**
   - Server-side rendered React via Tuono
   - Dense, readable, workflow-oriented UI (not marketing-driven)
   - Administrative UI planned

3. **Authentication & Authorization**
   - OAuth2/OIDC via Auth0
   - Supported providers: Apple, Microsoft, Google, Twitch, Facebook
   - Role-based access control (user permissions)

4. **Content Moderation**
   - OpenAI Moderation API for image classification
   - Admin tooling for moderation actions

5. **Infrastructure**
   - Cloud-native monorepo
   - PostgreSQL for persistence
   - Redis for caching/events
   - Terraform-managed infrastructure
   - Targeting Azure Free tier + personal server deployment

6. **Real-time Features (Planned)**
   - Automatic WebSocket or SSE updates for threads/watched threads

## Scope
- **In Scope:** Backend API, frontend UI, image processing pipeline, database schema, migrations, infrastructure as code, CI/CD
- **Out of Scope (for now):** Time-series database for archived posts, advanced analytics, mobile apps

## Success Criteria
- Functional imageboard with board/thread/post hierarchy
- Working OAuth login flow
- Image upload and optimization pipeline
- Admin UI for moderation
- Deployable stack via Terraform

## Project Maturity Warning
> **This repository is far from its first release. Expect nothing to work here just yet.**
