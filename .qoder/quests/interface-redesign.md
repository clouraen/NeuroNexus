# NeuroNexus Interface Redesign

## Overview

This document defines the redesign of the NeuroNexus user interface to achieve a sleek, futuristic cyberpunk aesthetic inspired by the ABAGEN WORKFLOW infographic. The interface will feature a dark-themed environment with vibrant neon accents (cyan, purple, orange) and incorporate a prominent brain motif as a visual identity element.

## Design Principles

### Visual Identity
- **Cyberpunk Aesthetics**: Neon-lit, high-tech visual language that evokes a futuristic learning environment
- **Brain Motif Integration**: The brain serves as the central symbol representing neural connections, knowledge networks, and cognitive growth
- **Glowing Elements**: All interactive and important elements feature subtle glow effects to create depth and visual hierarchy
- **Minimalist Complexity**: Clean layouts with sophisticated details that reveal themselves on interaction

### User Experience Goals
- Create an immersive, futuristic learning environment that motivates engagement
- Maintain high readability despite dark theme through proper contrast and glow effects
- Provide clear visual feedback for all interactive elements
- Ensure consistent visual language across all screens and components

## Color System

### Primary Palette

| Color Name | Hex Code | Usage | Glow Effect |
|------------|----------|-------|-------------|
| Deep Black | #000000 | Primary background | None |
| Dark Navy | #0a0a0a | Secondary backgrounds, cards | None |
| Neon Cyan | #00ffff | Primary brand color, highlights, metrics | 0 0 10px cyan |
| Neon Purple | #9d4edd | Secondary brand, borders, text | 0 0 10px purple |
| Bright Purple | #c77dff | Hover states, active elements | 0 0 20px purple |
| Neon Orange | #ff6b35 | Warnings, special accents | 0 0 10px orange |
| Hot Pink | #ff10f0 | Status indicators, alerts | 0 0 10px pink |

### Supporting Colors

| Color Name | Hex Code | Usage |
|------------|----------|-------|
| Light Purple | #e0aaff | Body text, labels |
| Muted Purple | rgba(157, 78, 221, 0.5) | Disabled states, placeholders |
| Gold | #ffd700 | Scores, achievements |
| White | #ffffff | Version text, secondary info |
| Light Gray | #cccccc | Email addresses, metadata |

## Typography

### Font Family Strategy

| Context | Font Stack | Weight | Size Guidelines |
|---------|-----------|--------|----------------|
| Headings | 'Orbitron', 'Courier New', monospace | Bold (700) | 2.5rem - 1.5rem |
| Body Text | 'Roboto', 'Arial', sans-serif | Regular (400) | 1rem - 0.875rem |
| Labels | 'Roboto', 'Arial', sans-serif | Semi-bold (600) | 0.75rem - 0.9rem |
| Monospace | 'Courier New', monospace | Regular (400) | Timer, code snippets |

### Text Styling Specifications

| Element Type | Size | Transform | Letter Spacing | Glow |
|--------------|------|-----------|----------------|------|
| App Title (NEURONEXUS) | 1.8rem | Uppercase | 0.15em | Cyan glow (intense) |
| Page Titles | 2.5rem | Uppercase | 0.2em | Purple glow |
| Section Titles | 1.5rem | Uppercase | 0.1em | Purple glow (subtle) |
| Tab Labels | 0.75rem | Uppercase | 0.15em | Inherits from parent |
| Body Text | 1rem | None | Normal | None |
| Metrics/Stats | 3rem | None | 0.05em | Cyan glow |

## Layout Architecture

### Screen Structure

The interface follows a three-tier vertical layout:

```
┌─────────────────────────────────────────────────┐
│              HEADER SECTION                     │
│  (Logo + Timer + Status)                        │
├─────────────────────────────────────────────────┤
│                                                 │
│                                                 │
│           MAIN CONTENT AREA                     │
│        (Dynamic page content)                   │
│                                                 │
│                                                 │
├─────────────────────────────────────────────────┤
│              FOOTER/DOCK BAR                    │
│         (Navigation icons)                      │
└─────────────────────────────────────────────────┘
```

### Header Section Design

#### Layout Composition
- **Container**: Full-width bar with dark background (rgba(10, 10, 10, 0.95))
- **Height**: 4rem (64px)
- **Border**: 2px solid neon purple at bottom
- **Shadow**: 0 2px 15px rgba(157, 78, 221, 0.3)
- **Backdrop**: Blur effect (10px) for depth

#### Three-Column Grid

**Left Section** (33% width)
- Lightning bolt icon (⚡) in neon cyan with pulsing glow animation
- App name "NEURONEXUS" in large glowing cyan letters
- Spacing: 0.75rem gap between icon and text

**Center Section** (34% width)
- Digital timer display: "00:00:00"
- Font: Monospace (Courier New)
- Color: Glowing purple
- Size: 1.2rem
- Letter spacing: 0.1em
- Purpose: Study session timer

**Right Section** (33% width)
- Status dot (●) in hot pink with "ONLINE" label
- Separator: " | "
- Version identifier: "v0.1.0" in white
- Alignment: Right-aligned

### Main Content Area

#### Container Specifications
- **Background**: Linear gradient from deep black to dark navy (135deg)
- **Padding**: 2rem on all sides
- **Bottom Padding**: 6rem (to clear footer)
- **Overflow**: Vertical scroll enabled
- **Min-height**: calc(100vh - header height - footer height)

#### Content Grid System

The main area supports multiple layout patterns based on page type:

**Two-Panel Layout** (Profile/Settings pages)
```
┌─────────────────────────────────────────┐
│  [Panel Title 1]    [Panel Title 2]     │
│  ┌──────────────┐  ┌──────────────┐     │
│  │              │  │              │     │
│  │   Panel 1    │  │   Panel 2    │     │
│  │   Content    │  │   Content    │     │
│  │              │  │              │     │
│  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────┘
```
- Panel gap: 2rem
- Each panel: 50% width minus gap
- Responsive: Stack vertically on screens < 768px

**Dashboard Grid** (Statistics cards)
- Grid: Auto-fit columns, minimum 200px, maximum 1fr
- Gap: 1.5rem
- Cards: Equal height, centered content

### Footer/Dock Bar Design

#### Structural Layout
- **Position**: Fixed at bottom of viewport
- **Width**: 100%
- **Height**: 5rem (80px)
- **Background**: rgba(10, 10, 10, 0.95) with 10px blur
- **Border Top**: 2px solid neon purple
- **Shadow**: 0 -5px 20px rgba(157, 78, 221, 0.3)
- **Z-index**: 1000 (always on top)

#### Icon Grid Layout
- **Display**: Flexbox with space-around distribution
- **Icon spacing**: Evenly distributed across width
- **Minimum width per icon**: 80px
- **Padding**: 0.75rem vertical

#### Individual Dock Items

Each dock item contains:
- Icon (1.6rem size) with drop-shadow filter
- Label text below icon (0.75rem)
- 0.4rem gap between icon and label
- Hover state: Scale icon to 1.1x, intensify glow
- Active state: Background tint rgba(199, 125, 255, 0.15)
- Active indicator: 3px glowing line at bottom

**Icon Inventory**

| Item | Icon | Label | Route |
|------|------|-------|-------|
| Dashboard | 📊 (Bar chart) | DASHBOARD | / |
| Knowledge Trails | 🗺️ (Map) | TRILHA | /trilhas |
| Questions | ❓ (Question mark) | QUESTÕES | /questoes |
| Essays | ✏️ (Pencil) | REDAÇÕES | /redacoes |
| Profile | 👤 (User silhouette) | PERFIL | /perfil |
| System Apps | Varies | FINDER/TERMINAL/etc | System integration |
| Trash | 🗑️ (Trash can) | TRASH | Delete actions |

## Component Specifications

### Profile Panel Component

#### Purpose
Display user identity and key engagement metrics

#### Layout Structure

**Title Bar**
- Text: "PERFIL"
- Size: 1.8rem
- Color: Neon purple with glow
- Transform: Uppercase
- Margin bottom: 1rem

**Container Card**
- Border: 1px solid neon purple (thin, rounded)
- Border radius: 8px
- Padding: 2rem
- Background: Linear gradient (135deg, rgba(10,10,10,0.9), rgba(0,0,0,0.9))
- Shadow: 0 0 15px rgba(157, 78, 221, 0.2)

**Avatar Section**
- Shape: Circle (80px diameter)
- Border: 2px solid neon purple
- Icon: User silhouette (👤) at 3rem size
- Shadow: 0 0 20px rgba(157, 78, 221, 0.5)
- Position: Top of card, centered or left-aligned with info

**User Information**
- Name/Role: "Estudante" in white, 1.2rem
- Email: "estudante@neuronexus.app" in light gray, 0.9rem
- Spacing: 0.5rem between name and email

**Metrics Section**
- Layout: Horizontal row with equal-width columns
- Gap: 2rem between metrics
- Margin top: 2rem from user info

**Individual Metric Display**
```
  [Value]     [Value]
  [Label]     [Label]
```
- Value: 2rem size, neon cyan color with glow
- Label: 0.9rem size, neon purple color
- Alignment: Center-aligned within column

**Metric Data Points**
1. Streak Days: "0" / "Dias de sequência"
2. Study Time: "0h" / "Tempo de estudo"

### Settings Panel Component

#### Purpose
Provide quick access to application configuration options

#### Layout Structure

**Title Bar**
- Text: "CONFIGURAÇÕES"
- Size: 1.8rem
- Color: Neon purple with glow
- Transform: Uppercase
- Margin bottom: 1rem

**Container Card**
- Same styling as Profile Panel container
- Border: 1px solid neon purple
- Border radius: 8px
- Padding: 2rem

**Settings Items List**
- Display: Vertical stack
- Item padding: 1rem vertical
- Divider: 1px solid rgba(157, 78, 221, 0.2) between items

**Individual Setting Item**
```
[Label Text]                    [✓]
```
- Layout: Flex with space-between justification
- Label: White text, 1rem size
- Checkbox/Toggle: Purple checkmark (✓) on the right
- Enabled state: Solid checkmark
- Disabled state: Empty box or grayed checkmark

**Current Settings**
1. "Notificações" - Enabled (purple checkmark visible)

### Cyber Card Component

#### Purpose
Reusable container for content blocks throughout the application

#### Visual Specifications

| Property | Value |
|----------|-------|
| Background | Linear gradient 135deg: rgba(10,10,10,0.9) → rgba(0,0,0,0.9) |
| Border | 1px solid neon purple |
| Border radius | 4px (subtle rounding) |
| Padding | 1.5rem |
| Box shadow | 0 0 15px rgba(157, 78, 221, 0.2) |
| Transition | all 0.3s ease |

#### Interactive States

**Hover State** (for clickable cards)
- Border color: Bright purple (#c77dff)
- Shadow: 0 0 25px rgba(199, 125, 255, 0.4)
- Transform: translateY(-2px)

**Active/Focus State**
- Border color: Neon cyan
- Shadow: Enhanced glow with cyan tint

### Neon Button Component

#### Purpose
Primary interactive element for user actions

#### Base Styling

| Property | Value |
|----------|-------|
| Background | Transparent |
| Border | 2px solid (color varies by variant) |
| Padding | 0.75rem horizontal, 1.5rem vertical |
| Font size | 1rem |
| Font family | Inherit from system |
| Text transform | Uppercase |
| Letter spacing | 0.1em |
| Cursor | Pointer |
| Transition | all 0.3s ease |

#### Variant Specifications

**Primary Variant**
- Border color: Neon purple (#9d4edd)
- Text color: Neon purple
- Text shadow: 0 0 10px purple
- Box shadow: 0 0 10px rgba(157, 78, 221, 0.3)
- Hover: Border → bright purple, enhanced glow, lift 2px

**Secondary Variant**
- Border color: Neon cyan (#00ffff)
- Text color: Neon cyan
- Text shadow: 0 0 10px cyan
- Box shadow: 0 0 10px rgba(0, 255, 255, 0.3)
- Hover: Border → bright cyan, enhanced glow

**Danger Variant** (for destructive actions)
- Border color: Neon orange (#ff6b35)
- Text color: Neon orange
- Text shadow: 0 0 10px orange
- Hover: Intensified red-orange glow

### Neon Input Component

#### Purpose
Text input fields for forms and search

#### Styling Specifications

| Property | Value |
|----------|-------|
| Background | rgba(10, 10, 10, 0.8) |
| Border | 2px solid neon purple |
| Color | Light purple (#e0aaff) |
| Padding | 0.75rem 1rem |
| Font size | 1rem |
| Font family | Inherit |
| Width | 100% (responsive) |
| Box shadow | 0 0 10px rgba(157, 78, 221, 0.2) |
| Transition | all 0.3s ease |

#### Focus State
- Border color: Bright purple (#c77dff)
- Shadow: 0 0 20px bright purple, 0 0 30px bright purple
- Outline: None (custom glow instead)

#### Placeholder Styling
- Color: rgba(157, 78, 221, 0.5)
- Font style: Italic (optional)

### Progress Bar Component

#### Purpose
Visualize completion percentage for learning activities

#### Structure

**Container**
- Width: 100%
- Height: 20px
- Background: rgba(10, 10, 10, 0.8)
- Border: 1px solid neon purple
- Border radius: 2px
- Position: Relative
- Overflow: Hidden

**Progress Fill**
- Height: 100%
- Background: Linear gradient 90deg: neon purple → bright purple
- Box shadow: 0 0 10px purple, 0 0 20px purple
- Transition: width 0.3s ease
- Width: Dynamic (0-100%)

**Label Elements**
- Top label: Description text, 0.9rem, light purple
- Bottom label: Percentage text, right-aligned, 0.9rem

## Brain Motif Integration

### Strategic Placement Options

#### Option 1: Header Integration
- Position: Next to "NEURONEXUS" text in header
- Size: 1.5rem
- Style: Stylized brain icon in neon cyan
- Animation: Subtle pulse (2s cycle)
- Glow: 0 0 15px cyan

#### Option 2: Background Watermark
- Position: Centered in main content area, absolute positioning
- Size: 40% of viewport width
- Opacity: 0.03-0.05 (very subtle)
- Color: Neon purple outline
- Z-index: Below all content (background layer)
- Style: Geometric, stylized brain with neural connections

#### Option 3: Loading State
- Display: During data fetch operations
- Animation: Rotating neural network pattern
- Position: Center of loading area
- Size: 80px
- Glow: Pulsing cyan/purple alternation

#### Recommended Approach
Combination of Option 1 (header) and Option 2 (watermark):
- Small animated brain icon in header for constant brand presence
- Large watermark in background for atmospheric depth without interference

### Brain Icon Design Specifications

#### Visual Characteristics
- Style: Geometric/minimalist interpretation
- Line weight: 2px
- Color: Neon cyan primary, purple accents for neural pathways
- Elements: Brain outline with 3-5 visible neural connection lines
- Symmetry: Slightly asymmetric for organic feel

#### Animation Specifications

**Pulse Animation** (Header icon)
```
Keyframes (2s duration, infinite loop):
  0%, 100%: 
    - Shadow: 0 0 10px cyan
    - Scale: 1.0
  50%: 
    - Shadow: 0 0 20px cyan, 0 0 30px cyan
    - Scale: 1.05
```

**Neural Flow Animation** (Watermark)
```
Keyframes (8s duration, infinite loop):
  0%: Neural lines opacity at 0.3
  50%: Neural lines opacity at 0.8
  100%: Neural lines opacity at 0.3
  
Stagger: Each line animates with 0.5s delay
```

## Interactive Elements

### Hover Effects Specification

All interactive elements must provide clear hover feedback:

#### Buttons
- Glow intensification: Base shadow × 2
- Color shift: Base color → Brighter variant
- Transform: translateY(-2px) for lift effect
- Transition: 0.3s ease

#### Navigation Items (Dock)
- Icon scale: 1.0 → 1.1
- Label glow: None → subtle glow
- Background: Transparent → tinted (rgba(157, 78, 221, 0.1))
- Transition: 0.2s ease

#### Cards (Clickable)
- Border intensity: Increase brightness
- Shadow spread: +10px
- Vertical lift: -2px
- Transition: 0.3s ease

#### Inputs
- Border color shift on focus
- Glow expansion
- Placeholder fade out on type

### Timer Functionality

#### Purpose
Track active study session duration

#### Behavior Specifications

**Display Format**
- Pattern: HH:MM:SS
- Example: "00:00:00" → "01:23:45"
- Font: Monospace for alignment stability

**State Management**
- Initial: "00:00:00" (idle)
- Active: Incrementing every second
- Paused: Frozen at current time
- Reset: Return to "00:00:00"

**Visual Feedback**
- Idle: Steady purple glow
- Active: Pulsing glow animation (slower, 3s cycle)
- Paused: Reduced opacity (0.7)
- Approaching milestone (e.g., 1 hour): Color shift to cyan

**Interaction**
- Click to start/pause
- Right-click or long-press to reset
- Optional: Tooltip showing "Click to start timer"

### Toggle/Checkbox Component

#### Visual Design

Settings use a checkmark-based toggle system:

**Enabled State**
- Symbol: "✓" (checkmark)
- Color: Neon purple
- Size: 1.2rem
- Glow: 0 0 8px purple

**Disabled State**
- Symbol: "○" (empty circle) or no symbol
- Color: Muted purple rgba(157, 78, 221, 0.3)
- Size: 1.2rem
- No glow

**Hover State**
- Scale: 1.1
- Glow intensification
- Cursor: Pointer

## Responsive Behavior

### Breakpoint Strategy

| Breakpoint | Width | Layout Adjustments |
|------------|-------|-------------------|
| Desktop | ≥ 1024px | Full three-column header, two-panel content layouts |
| Tablet | 768px - 1023px | Maintain header layout, stack content panels vertically |
| Mobile | < 768px | Compact header (smaller text), single-column content, dock remains fixed |

### Mobile Adaptations

#### Header Modifications
- Logo text: Reduce to "NNX" or icon only
- Timer: Maintain visibility, reduce size to 1rem
- Status: Show only online dot, hide version text
- Height: Reduce to 3.5rem

#### Content Area
- Panel grid: Force single column
- Card padding: Reduce to 1rem
- Font sizes: Scale down by 10-15%
- Spacing: Reduce gaps from 2rem to 1rem

#### Dock Bar
- Icon size: Maintain at 1.6rem
- Label size: Reduce to 0.65rem
- System app icons: Hide on mobile (show only core app navigation)
- Item width: Reduce minimum to 60px

### Touch Interactions

For touch-enabled devices:

- Increase touch target size to minimum 44×44px
- Add active press state (scale down to 0.95)
- Implement swipe gestures for navigation (optional enhancement)
- Long-press on timer for reset functionality

## Accessibility Considerations

### Color Contrast

Despite dark theme, maintain WCAG AA compliance:

- Body text (light purple on black): Minimum 4.5:1 ratio
- Headings (cyan/purple on black): Minimum 3:1 ratio
- Interactive elements: Clear visual distinction from static content

### Text Readability

- Minimum font size: 0.75rem (12px)
- Line height: 1.5 for body text, 1.2 for headings
- Glow effects: Enhance but not obscure readability
- Letter spacing: Improve legibility for uppercase text

### Keyboard Navigation

- All interactive elements must be keyboard accessible
- Focus states: Visible cyan outline (2px) with glow
- Tab order: Logical flow (header → content → footer)
- Shortcut keys: Consider implementing for power users

### Screen Reader Support

- Semantic HTML structure
- ARIA labels for icon-only buttons
- Live region announcements for timer updates
- Alt text for brain motif (decorative: aria-hidden="true")

## Animation and Motion

### Performance Guidelines

- Use CSS transforms and opacity for animations (GPU-accelerated)
- Avoid animating width, height, or layout properties
- Respect user preference: prefers-reduced-motion media query
- Keep animation durations under 400ms for interactions

### Animation Inventory

| Element | Animation | Duration | Easing |
|---------|-----------|----------|--------|
| Logo icon pulse | Glow intensity | 2s | ease-in-out |
| Hover lift | translateY | 0.3s | ease |
| Border glow | Box-shadow | 0.3s | ease |
| Page transitions | Fade | 0.2s | ease |
| Brain pulse | Scale + glow | 2s | ease-in-out |
| Neural flow | Opacity | 8s | ease-in-out |
| Tab indicator | Box-shadow pulse | 2s | ease-in-out |
| Input focus | Border + shadow | 0.3s | ease |
| Button press | Scale | 0.1s | ease-out |

### Reduced Motion Alternative

For users with motion sensitivity (prefers-reduced-motion: reduce):

- Disable all pulse animations
- Replace movement with subtle opacity changes
- Maintain hover states without transforms
- Keep essential feedback (border color changes)

## Page-Specific Layouts

### Profile Page Layout

Two-panel horizontal layout on desktop, stacked on mobile:

**Left Panel: Profile Information**
- Title: "PERFIL" (as specified in component section)
- Content: Avatar, user details, metrics
- Width: 50% (desktop), 100% (mobile)

**Right Panel: Settings**
- Title: "CONFIGURAÇÕES"
- Content: Toggle settings list
- Width: 50% (desktop), 100% (mobile)
- Mobile: Appears below profile panel

**Spacing**
- Gap between panels: 2rem (desktop)
- Vertical gap: 1.5rem (mobile)

### Dashboard Page Layout

**Title Section**
- "DASHBOARD" page title
- Margin bottom: 2rem

**Statistics Grid**
- Layout: 3-column grid (auto-fit, minmax(200px, 1fr))
- Gap: 1.5rem
- Cards: Cyber card component with centered content
- Content structure per card:
  - Large metric value (3rem, cyan)
  - Label below (1rem, purple)

**Responsive Behavior**
- Desktop: 3 columns
- Tablet: 2 columns
- Mobile: 1 column

### Essays/Questions List Pages

**Header Section**
- Page title
- Search input (full-width neon input component)
- Action button (e.g., "Nova Redação")
- Spacing: 1rem between elements

**List Section**
- Item cards: Cyber card component
- Spacing: 1rem vertical gap between cards
- Hover: Entire card interactive
- Content varies by item type (essay vs question)

### Detail Pages (Essay/Question)

**Title Section**
- Large title with back button
- Metadata row (date, type, status)

**Content Section**
- Full-width content area
- Scrollable if content exceeds viewport
- Preserved formatting for essays

**Action Section**
- Bottom-aligned action buttons
- Margin top: 2rem

## Implementation Phases

This design should be implemented in logical phases:

### Phase 1: Foundation
- Update color system and CSS variables
- Implement brain motif assets (icon and watermark)
- Refactor header component with new three-column layout
- Update typography system

### Phase 2: Core Components
- Enhanced Cyber Card component with hover states
- Neon Button variants refinement
- Neon Input with focus animations
- Progress Bar component
- Toggle/Checkbox component for settings

### Phase 3: Layout Updates
- Footer/Dock bar redesign with icon grid
- Profile page two-panel layout
- Dashboard statistics grid
- Responsive breakpoint implementations

### Phase 4: Interactions
- Timer functionality (start/pause/reset)
- Hover effect implementations across all interactive elements
- Keyboard navigation and focus states
- Animation system activation

### Phase 5: Polish
- Brain motif animations
- Loading states
- Transition effects between pages
- Accessibility audit and fixes
- Performance optimization

## Design Validation Criteria

### Visual Consistency Checklist
- [ ] All text follows typography specifications
- [ ] Color usage adheres to defined palette
- [ ] Glow effects consistent across similar elements
- [ ] Border radii uniform (4px cards, 8px panels, 50% avatars)
- [ ] Spacing follows 0.5rem increment system

### Interaction Quality Checklist
- [ ] All buttons provide clear hover feedback
- [ ] Focus states visible for keyboard navigation
- [ ] Animations smooth at 60fps
- [ ] Loading states present for async operations
- [ ] Error states clearly communicate issues

### Brand Alignment Checklist
- [ ] Brain motif integrated and visible
- [ ] Cyberpunk aesthetic maintained throughout
- [ ] "NEURONEXUS" branding consistent
- [ ] Neon glow effects enhance rather than distract
- [ ] Futuristic learning environment feel achieved

### Accessibility Checklist
- [ ] WCAG AA contrast ratios met
- [ ] Keyboard navigation complete
- [ ] Screen reader compatible
- [ ] Reduced motion support implemented
- [ ] Touch targets adequate for mobile (44×44px minimum)

## Future Enhancements

Potential features for post-MVP iterations:

### Advanced Brain Motif
- Interactive neural network visualization showing learning progress
- Nodes representing completed topics, connections showing relationships
- Animated growth as user completes more content

### Theme Customization
- User-selectable accent color (cyan, purple, orange, green)
- Intensity controls for glow effects
- Option to reduce or eliminate animations

### Enhanced Timer
- Pomodoro technique integration (25min sessions)
- Session history tracking
- Daily/weekly study time reports
- Goal setting and progress visualization

### Ambient Animations
- Floating particles in background
- Subtle grid pattern overlays
- Dynamic background color shifts based on time of day

### Gamification Elements
- Achievement badges with special glow effects
- Streak visualizations with flame/lightning effects
- Level progression indicator in header
- Leaderboard with neon-styled rankings