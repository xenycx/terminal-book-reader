# Draft: Preferences Page

## Core Objective
Add a dedicated Preferences/Settings page to configure app behavior and reading experience.

## Proposed Settings (Scope)
- **Theme**: Light / Dark (Already implemented, move to UI)
- **Text Alignment**: Center / Left / Right (New feature to implement)
- **Margin Width**: Configurable via slider/number (Exposes existing `+/-` functionality)

## Creative Additions (To Confirm)
- **Startup View**: Library vs Last Read Book (New)
- **Search Case Sensitivity**: Case Sensitive / Insensitive toggle (Future feature placeholder)
- **Status Bar**: Show / Hide (New)
- **Target Reading Speed**: WPM goal for ETA calculations (New)

## Technical Approach (Pending)
- Add `AppMode::Preferences`
- Create `src/ui/preferences.rs` for rendering the form
- Update `AppConfig` serialization to handle new fields
