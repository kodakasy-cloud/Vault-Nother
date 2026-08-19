[README_VaultNote_EN.md](https://github.com/user-attachments/files/31219353/README_VaultNote_EN.md)
# 🗄️ VaultNote

> **A personal digital vault for organizing, storing, and finding what matters.**

**VaultNote** is a desktop application written in **Rust**, designed to provide a personal and centralized space for storing and organizing different kinds of content.

The goal is to keep **notes, screenshots, photos, images, files, links, and other important information** in one place, with a simple interface, local storage, and an architecture built to grow over time.

VaultNote aims to combine a modern user experience with the performance, reliability, and safety offered by the Rust ecosystem.

---

## 📌 Table of Contents

- [About](#-about)
- [Project Goals](#-project-goals)
- [Features](#-features)
- [Interface](#-interface)
- [Project Status](#-project-status)
- [Technology Stack](#-technology-stack)
- [Architecture](#-architecture)
- [Project Structure](#-project-structure)
- [Installing Rust](#-installing-rust)
- [Running the Project](#-running-the-project)
- [Core Dependencies](#-core-dependencies)
- [Usage Examples](#-usage-examples)
- [Development Roadmap](#-development-roadmap)
- [Future Features](#-future-features)
- [Project Principles](#-project-principles)
- [Contributing](#-contributing)
- [License](#-license)

---

# 📖 About

VaultNote is planned as a **personal digital vault**.

Instead of spreading important information across different applications, folders, and services, VaultNote aims to centralize everything inside a single desktop application.

The application is planned to support:

- 📝 Creating and organizing notes
- 🖼️ Storing images, screenshots, and photos
- 📁 Importing and managing files
- 🔗 Saving important links
- ⭐ Marking content as favorite
- 🔎 Searching across stored content
- 🗂️ Organizing items into categories
- 🗑️ Moving deleted items to a trash system

The main idea is to keep the application simple for everyday use while maintaining a clean and modular architecture that can evolve gradually.

---

# 🎯 Project Goals

The primary goal of VaultNote is to create a personal application where users can:

> **Save, organize, and find important information in a simple and centralized way.**

The project will prioritize:

- Clean and organized code
- Modular architecture
- Local-first storage
- Maintainability
- Gradual expansion
- Clear and intuitive interfaces
- Reusable components
- Separation between UI, application logic, and storage

---

# ✨ Features

## 📝 Notes

### Planned

The notes system is expected to support:

- Creating notes
- Editing notes
- Deleting notes
- Adding titles
- Writing content
- Assigning categories
- Searching notes
- Viewing recent notes
- Marking notes as favorites

Example:

```text
Title:
Project ideas

Category:
Programming

Content:
Build a desktop application using Rust,
Iced, and SQLite.
```

---

## 🖼️ Images and Screenshots

### Planned

Users will be able to:

- Import images
- Store screenshots
- Store photos
- Organize images
- Search images
- View basic information
- Assign categories
- Mark content as favorite

Possible metadata:

```text
ID
Name
Path
Type
Category
Created date
Modified date
```

Initially, files will remain stored locally on the user's computer, while SQLite will manage their metadata.

---

## 📁 Files

### Planned

The file system is expected to support:

- Importing files
- Organizing files by category
- Viewing basic file information
- Searching files
- Marking files as favorites
- Moving files to the trash
- Opening files with the operating system

Possible supported formats include:

```text
PDF
TXT
DOCX
ZIP
JSON
CSV
Other compatible formats
```

The initial list of supported formats may evolve as development progresses.

---

## 🔗 Links

### Planned

VaultNote may also support storing important links.

Example:

```text
Title:
Rust Documentation

URL:
https://www.rust-lang.org/

Category:
Programming
```

---

## 🗂️ Organization

The application is planned to include features that make stored content easier to navigate:

- Categories
- Favorites
- Recent items
- Global search
- Filters
- Trash
- Future tag support

---

# 🖥️ Interface

The graphical interface is planned with **Iced**.

The initial layout is expected to follow a structure similar to:

```text
┌──────────────────────────────────────────────────────┐
│ SIDEBAR       │              CONTENT                 │
│               │                                      │
│ 🏠 Home       │       Dashboard / Current View       │
│ 📝 Notes      │                                      │
│ 🖼️ Images     │                                      │
│ 📁 Files      │                                      │
│ ⭐ Favorites  │                                      │
│ 🗑️ Trash      │                                      │
│ ⚙️ Settings   │                                      │
│               │                                      │
└──────────────────────────────────────────────────────┘
```

## Planned Screens

| Screen | Description | Status |
|---|---|---|
| 🏠 Home | Main dashboard | ⏳ Planned |
| 📝 Notes | Note management | ⏳ Planned |
| 🖼️ Images | Images and screenshots | ⏳ Planned |
| 📁 Files | File management | ⏳ Planned |
| ⭐ Favorites | Favorited content | ⏳ Planned |
| 🗑️ Trash | Removed items | ⏳ Planned |
| ⚙️ Settings | Application preferences | ⏳ Planned |

> Screen names, organization, and details may change as the project evolves.

---

# 📊 Project Status

VaultNote is currently in the **early planning and foundation-building stage**.

To clearly distinguish ideas from implemented functionality, the project uses the following status labels:

| Status | Meaning |
|---|---|
| ✅ Completed | Implemented and working |
| 🚧 In Progress | Currently under development |
| ⏳ Planned | Defined but not started |
| 💡 Future | Idea for a later version |

## Current Progress

| Area | Status |
|---|---|
| Project planning | 🚧 In Progress |
| Initial architecture | 🚧 In Progress |
| Graphical interface | ⏳ Planned |
| Screen navigation | ⏳ Planned |
| Notes system | ⏳ Planned |
| SQLite database | ⏳ Planned |
| File management | ⏳ Planned |
| Categories | ⏳ Planned |
| Search | ⏳ Planned |
| Favorites | ⏳ Planned |
| Trash | ⏳ Planned |

> **Important:** Features marked as planned should not be considered implemented.

---

# 🦀 Technology Stack

VaultNote is planned primarily around:

| Technology | Purpose |
|---|---|
| Rust | Main programming language |
| Iced | Desktop graphical interface |
| SQLite | Local database |
| SQLx | Database communication |
| Tokio | Asynchronous operations when needed |

Additional libraries may be introduced as the project grows.

The priority is to keep the dependency tree reasonable and avoid adding unnecessary libraries.

---

# 🏗️ Architecture

The project is planned around a separation of responsibilities:

```text
┌──────────────────────────────┐
│            VIEWS             │
│                              │
│   Screens and UI elements    │
└───────────────┬──────────────┘
                │
                ▼
┌──────────────────────────────┐
│         APP / STATE          │
│                              │
│ Application state and flow   │
└───────────────┬──────────────┘
                │
                ▼
┌──────────────────────────────┐
│           SERVICES           │
│                              │
│     Business logic layer     │
└───────────────┬──────────────┘
                │
                ▼
┌──────────────────────────────┐
│          DATABASE            │
│                              │
│        SQLite + SQLx         │
└──────────────────────────────┘
```

## Separation of Responsibilities

### `views/`

Responsible for application screens and visual composition.

Examples:

```text
home.rs
notes.rs
images.rs
files.rs
favorites.rs
trash.rs
settings.rs
```

Views should focus primarily on building the user interface.

### `components/`

Contains reusable UI components.

Examples:

```text
sidebar
header
search
item_card
```

The goal is to avoid unnecessary duplication between screens.

### `models/`

Represents the application's data structures.

```rust
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
}
```

Other models may represent notes, files, categories, favorites, and additional content types.

### `database/`

Responsible for database initialization and communication.

Possible responsibilities:

```text
Create connections
Initialize the database
Execute queries
Create tables
Manage migrations in the future
```

### `services/`

Responsible for application and business logic.

Examples:

```text
Create notes
Edit notes
Delete notes
List files
Import files
Search content
```

## Expected Application Flow

```text
User
  │
  ▼
Interface
  │
  ▼
Message
  │
  ▼
Update / State
  │
  ▼
Service
  │
  ▼
Database / Storage
```

This separation is intended to make the application easier to maintain and expand.

---

# 📁 Project Structure

The proposed initial structure is:

```text
vaultnote/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
│
├── assets/
│
├── storage/
│   ├── images/
│   └── files/
│
├── database/
│   └── vaultnote.db
│
└── src/
    │
    ├── main.rs
    ├── app.rs
    ├── message.rs
    ├── state.rs
    │
    ├── views/
    │   ├── mod.rs
    │   ├── home.rs
    │   ├── notes.rs
    │   ├── images.rs
    │   ├── files.rs
    │   ├── favorites.rs
    │   ├── trash.rs
    │   └── settings.rs
    │
    ├── components/
    │   ├── mod.rs
    │   ├── sidebar.rs
    │   ├── header.rs
    │   ├── item_card.rs
    │   └── search.rs
    │
    ├── models/
    │   ├── mod.rs
    │   ├── note.rs
    │   ├── file.rs
    │   └── category.rs
    │
    ├── database/
    │   ├── mod.rs
    │   └── connection.rs
    │
    └── services/
        ├── mod.rs
        ├── notes.rs
        └── files.rs
```

> The structure may change as the project grows. The goal is to start with a clear organization without introducing unnecessary complexity.

---

# 🚀 Installing Rust

To build or contribute to VaultNote, Rust must be installed.

The recommended installation method is through the official Rust installer:

urlrustup.rshttps://rustup.rs

After installation, verify that Rust is available:

```bash
rustc --version
```

Also verify Cargo:

```bash
cargo --version
```

---

# ▶️ Running the Project

## 1. Clone the repository

Once the repository is available:

```bash
git clone <REPOSITORY_URL>
```

Enter the project directory:

```bash
cd vaultnote
```

## 2. Run the application

```bash
cargo run
```

Cargo will:

1. Download the required dependencies
2. Compile the project
3. Launch the application

## 3. Build without running

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

---

# 📦 Core Dependencies

The initial dependencies are expected to include libraries similar to:

```toml
[dependencies]

iced = "..."

sqlx = { version = "...", features = ["sqlite"] }

tokio = {
    version = "...",
    features = ["full"]
}
```

Specific versions will be defined during project setup.

Additional dependencies may be introduced for features such as:

- File selection
- Path handling
- Date and time management
- Identifiers
- Error handling
- Serialization

The project should avoid adding dependencies solely for convenience when the same functionality can be reasonably implemented with the standard library or existing dependencies.

---

# 📝 Usage Examples

## Saving a Note

A possible workflow:

```text
1. User opens VaultNote

2. Clicks:
   + New Note

3. Defines:
   Title
   Category
   Content

4. Clicks:
   Save

5. The application:
   ├── Validates the data
   ├── Creates the note model
   ├── Calls the responsible service
   └── Saves the data to SQLite
```

## Example Organization

A user's content could conceptually be organized like this:

```text
📁 Programming
│
├── 📝 Project ideas
├── 📝 Important commands
├── 🖼️ Reference screenshot
└── 📄 Document.pdf

📁 Studies
│
├── 📝 Rust notes
├── 📝 Exercise ideas
└── 📄 Study material

📁 Personal
│
├── 🖼️ Photos
├── 📝 Important information
└── 🔗 Saved links
```

The exact organization model will be defined as the data architecture evolves.

---

# 🗺️ Development Roadmap

The project will be developed in stages to allow steady and controlled growth.

## 🧱 Phase 1 — Foundation

**Goal:** Create the minimum working structure of the application.

- [ ] Create the Rust project
- [ ] Configure Iced
- [ ] Create the main application window
- [ ] Create the initial module structure
- [ ] Create the `App`
- [ ] Create a basic message system
- [ ] Create state management
- [ ] Create basic navigation between screens

## 🎨 Phase 2 — Interface

**Goal:** Build the main visual structure of VaultNote.

- [ ] Create the sidebar
- [ ] Create visual navigation
- [ ] Create the dashboard
- [ ] Create the notes screen
- [ ] Create the initial images screen
- [ ] Create the initial files screen
- [ ] Create reusable components
- [ ] Create the header
- [ ] Create a button for adding content
- [ ] Improve layout and spacing

## 🗄️ Phase 3 — Database

**Goal:** Implement local storage using SQLite.

- [ ] Configure SQLite
- [ ] Configure SQLx
- [ ] Create the database connection
- [ ] Create the initial table structure
- [ ] Create the notes model
- [ ] Save notes
- [ ] List notes
- [ ] Edit notes
- [ ] Delete notes

## 📁 Phase 4 — Files

**Goal:** Add file storage and management.

- [ ] Create a file import system
- [ ] Create the storage directory
- [ ] Create an image storage directory
- [ ] Create a general file storage directory
- [ ] Save metadata to SQLite
- [ ] List files
- [ ] Display basic information
- [ ] Implement opening files

## 🗂️ Phase 5 — Organization

**Goal:** Improve navigation and content discovery.

- [ ] Create categories
- [ ] Implement favorites
- [ ] Create search
- [ ] Create global search
- [ ] Implement trash
- [ ] Create filters
- [ ] Create a recent items list

## 🚀 Phase 6 — Future Improvements

**Goal:** Expand the application after the core foundation is stable.

- [ ] Add a tag system
- [ ] Add data backups
- [ ] Add data export
- [ ] Add a light theme
- [ ] Add a dark theme
- [ ] Explore optional encryption
- [ ] Improve security
- [ ] Explore future device synchronization

---

# 🔮 Future Features

The features below are ideas for future versions and **are not part of the current implementation**.

## 🏷️ Tags

Allow multiple tags to be attached to the same piece of content.

```text
#rust
#programming
#studies
#project
```

## 💾 Backups

Possible backup options:

```text
Database backup
File backup
Full backup
```

## 📤 Exporting

Possible future export features:

```text
Export notes
Export application data
Export categories
Create backup packages
```

## 🎨 Themes

Future support may include:

- Light theme
- Dark theme
- Operating system preference detection

## 🔐 Optional Encryption

A future possibility is allowing selected data to be protected by additional security mechanisms.

This feature should be carefully designed before implementation.

## 🔄 Device Synchronization

A long-term possibility is synchronization between different devices:

```text
Desktop
   │
   ▼
Synchronization System
   │
   ├── Laptop
   │
   └── Other Devices
```

Device synchronization is **not part of the first version** of VaultNote.

---

# 🧠 Project Principles

## Simplicity

Start small.

```text
First:
A working application

Then:
Core features

Then:
Improvements

Then:
Expansion
```

## Organization

Responsibilities should remain clearly separated.

Avoid mixing:

```text
Interface
+
Database
+
Business Logic
+
File Handling
```

inside a single file or module.

Prefer clear boundaries such as:

```text
views/
services/
models/
database/
components/
```

## Gradual Evolution

New functionality should be added once the existing foundation is stable.

The goal is not to implement every idea immediately.

## Local-First Storage

The first version will prioritize:

```text
Desktop Application
+
SQLite
+
Local Files
```

Online synchronization and cloud-related features can be evaluated later.

---

# 🤝 Contributing

VaultNote is currently in an early stage.

As the repository becomes ready for collaboration, this section may include guidelines for:

- Opening issues
- Suggesting features
- Contributing code
- Commit conventions
- Code standards
- Pull requests

---

# 📄 License

The project's license has not yet been defined.

Once a license is selected, the repository should clearly describe the terms for use, modification, and distribution.

---

# 📌 Current Status

## 🚧 In Development

VaultNote is currently in the early stage of planning and building its foundation.

### Next Steps

```text
1. Create the Rust project
        ↓
2. Configure Iced
        ↓
3. Create the main window
        ↓
4. Organize the modules
        ↓
5. Create App, State, and Message
        ↓
6. Implement basic navigation
        ↓
7. Build the initial interface
```

---

# 🗄️ VaultNote

> **A place to keep what matters.**

The goal is to build an organized, modular, and extensible desktop application, starting with a solid foundation based on **Rust**, **Iced**, and **SQLite**.

**Status:** 🚧 In Development  
**Version:** Not defined yet  
**Initial Platform:** Desktop  
**Primary Language:** Rust 🦀
