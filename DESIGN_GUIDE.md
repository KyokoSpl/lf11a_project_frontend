# egui Material 3 Design Reference

## Color Palette

The egui frontend implements Material Design 3 with the following color scheme:

### Primary Colors
```
Primary:            #6750A4 (Deep Purple)
On Primary:         #FFFFFF (White)
Primary Container:  #EADDFF (Light Purple)
On Primary Cont:    #21005D (Very Dark Purple)
```

### Secondary Colors
```
Secondary:          #625B71 (Gray Purple)
On Secondary:       #FFFFFF (White)
Secondary Cont:     #E8DEF8 (Light Gray Purple)
```

### Surface & Background
```
Surface:            #FFFBFE (Off White)
On Surface:         #1C1B1F (Almost Black)
Surface Variant:    #E7E0EC (Light Gray)
On Surface Variant: #49454F (Dark Gray)
Outline:            #79747E (Medium Gray)
```

### Error States
```
Error:              #B3261E (Red)
On Error:           #FFFFFF (White)
```

## UI Components

### Tab Navigation (Top Bar)
```
┌─────────────────────────────────────────────────────────────┐
│  [🏢 Departments] [👥 Employees] [💰 Salary Grades]        │
│  ^^^^^^^^^^^^^^^^^^^^                                        │
│  Active tab: Purple background (#6750A4), white text        │
│  Inactive tabs: Light gray bg (#E7E0EC), dark text          │
│  Rounded corners: 20px radius                                │
└─────────────────────────────────────────────────────────────┘
```

### Form Card
```
┌─────────────────────────────────────────────────────────────┐
│  ╭───────────────────────────────────────────────────────╮  │
│  │ Department Name:                                      │  │
│  │ ┌─────────────────────────────────────────────────┐  │  │
│  │ │ Engineering                                     │  │  │
│  │ └─────────────────────────────────────────────────┘  │  │
│  │                                                       │  │
│  │ Head Employee ID (optional):                         │  │
│  │ ┌─────────────────────────────────────────────────┐  │  │
│  │ │ emp-123                                         │  │  │
│  │ └─────────────────────────────────────────────────┘  │  │
│  │                                                       │  │
│  │  ╭────────╮  ╭────────╮                             │  │
│  │  │ Create │  │ 🔄 Refresh │                         │  │
│  │  ╰────────╯  ╰────────╯                             │  │
│  ╰───────────────────────────────────────────────────────╯  │
└─────────────────────────────────────────────────────────────┘
    Card: White bg, 1px gray outline, 12px rounded corners
    Buttons: 20px rounded, purple for primary, gray for secondary
    16px internal padding
```

### List Item Card
```
┌─────────────────────────────────────────────────────────────┐
│  ╭───────────────────────────────────────────────────────╮  │
│  │  Engineering                         ╭──────╮ ╭──────╮ │
│  │  👤 Head: John Doe                   │ Edit │ │Delete│ │
│  │  👥 5 employee(s)                    ╰──────╯ ╰──────╯ │
│  ╰───────────────────────────────────────────────────────╯  │
│                                                               │
│  ╭───────────────────────────────────────────────────────╮  │
│  │  Sales                               ╭──────╮ ╭──────╮ │
│  │  👤 Head: Jane Smith                 │ Edit │ │Delete│ │
│  │  👥 3 employee(s)                    ╰──────╯ ╰──────╯ │
│  ╰───────────────────────────────────────────────────────╯  │
└─────────────────────────────────────────────────────────────┘
    Each card: 8px vertical spacing
    Title: 18px bold, dark gray (#1C1B1F)
    Subtitle: 14px, medium gray (#49454F)
    Buttons aligned right
```

### Employee Card (More Complex)
```
┌─────────────────────────────────────────────────────────────┐
│  ╭───────────────────────────────────────────────────────╮  │
│  │  John Doe                            ╭──────╮ ╭──────╮ │
│  │  📧 john.doe@company.com             │ Edit │ │Delete│ │
│  │  💼 Software Engineer                ╰──────╯ ╰──────╯ │
│  │  Department: Engineering                              │  │
│  │  Salary: E4 ($85000.00)                               │  │
│  ╰───────────────────────────────────────────────────────╯  │
└─────────────────────────────────────────────────────────────┘
    Name: 18px bold
    Details: 14px medium gray
    Icons: Emoji for visual appeal
    Layout: Vertical stack on left, buttons on right
```

### Salary Grade Card
```
┌─────────────────────────────────────────────────────────────┐
│  ╭───────────────────────────────────────────────────────╮  │
│  │  E4 - $85000.00                      ╭──────╮ ╭──────╮ │
│  │  Senior Software Engineer            │ Edit │ │Delete│ │
│  ╰───────────────────────────────────────────────────────╯  │
└─────────────────────────────────────────────────────────────┘
    Code + Salary: 18px bold
    Description: 14px medium gray
```

## Typography Scale

```
Headings:    28px  (Page titles like "Departments")
Titles:      18px  (Card titles, item names)
Body:        14px  (Labels, descriptions, buttons)
```

## Spacing System

```
Card padding:     16px (all sides)
Card margins:     8px (between cards)
Button padding:   12px horizontal, 10px vertical
Item spacing:     8px (vertical between elements)
Section spacing:  16px (between major sections)
```

## Interactive States

### Buttons
- **Rest**: Colored background, white text
- **Hover**: Slightly lighter background
- **Active**: Darker background
- **Disabled**: Gray background, gray text

### Text Inputs
- **Rest**: White background, gray outline
- **Focus**: Purple outline (#6750A4)
- **Error**: Red outline (#B3261E)

## Material 3 Principles Applied

1. **Elevation**: Cards appear to float above the surface with subtle borders
2. **Color Roles**: Clear distinction between primary, secondary, and tertiary actions
3. **Shape**: Rounded corners everywhere (20px for buttons, 12px for cards)
4. **Typography**: Clear hierarchy with three sizes
5. **Spacing**: Consistent 8px grid system
6. **Contrast**: High contrast text for readability
7. **State Layers**: Visual feedback for interactive elements

## Comparison with Iced Version

### Iced
- Rectangular buttons
- Light gray backgrounds (#F0F0F0)
- Simple borders (1px)
- Less spacing
- Standard system font sizes

### egui Material 3
- Rounded buttons (20px)
- Material color palette (purples, defined surfaces)
- Elevated cards with shadows effect
- Generous spacing (16px padding)
- Typography hierarchy (28/18/14px)

The Material 3 version provides a **more polished, modern, and professional** appearance while maintaining excellent functionality and performance.
