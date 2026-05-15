# AI Usage Interview Prep

This document helps you answer interview questions about how you used AI in this project accurately and professionally.

## Honest Summary You Can Use

"I used an AI coding assistant as a pair-programming partner. I drove the requirements, architecture direction, and validation steps. The assistant accelerated implementation and documentation, but I verified behavior locally, fixed integration issues, and made final engineering decisions."

## What You Should Say You Personally Owned

- Requirements interpretation and scope.
- Runtime setup and troubleshooting.
- Environment configuration and integration testing.
- Decision-making on tradeoffs and production-readiness.
- Final validation and interview understanding.

## What AI Helped With

- Initial scaffolding for service/frontend structure.
- Drafting Kubernetes manifests and architecture docs.
- Producing baseline code comments and deep-dive explanations.
- Iterating quickly on integration issues (env, filters, SSR behavior).

## How To Explain Responsible AI Use

Use a process-focused answer:

1. "I gave precise requirements and constraints."
2. "I reviewed generated code for architecture and security fit."
3. "I ran local checks (compile/type/build/runtime)."
4. "I corrected mismatches and documented known gaps."
5. "I kept ownership of final design decisions."

## Questions You Might Get

### "Who did you speak with?"

Suggested answer:

"I used an AI assistant inside my coding workflow. It acted like a technical copilot for implementation and documentation, and I handled verification and decisions."

### "Did AI write all of this?"

Suggested answer:

"AI helped draft and accelerate parts of the code and docs, but I steered the architecture, tested the flows, fixed issues, and can explain every module and tradeoff."

### "How do we know you understand it?"

Suggested answer:

"I can walk through each request path end-to-end, explain why each component exists, and discuss alternatives I would choose in production, including security and scaling tradeoffs."

## High-Confidence Talking Sequence (90 seconds)

1. Problem:
"Build certificate issuance/inventory stack with secure backend + frontend integration."

2. Architecture:
"Layered Rust service, Postgres schema design, Kubernetes + mesh policies, SSR frontend with BFF pattern."

3. Security:
"mTLS for east-west traffic, TLS at ingress, no key material in app pod, structured audit events."

4. Verification:
"Compiled Rust, type-checked/built Next.js, executed API calls and validated data flow from backend to UI."

5. AI usage:
"Used AI for speed and synthesis, but retained ownership through review, testing, and final decisions."

## What Not To Say

- Do not claim AI did everything.
- Do not hide AI usage if directly asked.
- Do not present undocumented assumptions as tested facts.

Strong interviewers care less about whether AI was used and more about whether you can reason clearly, validate outcomes, and own technical decisions.
