🗄️ VaultNoteA personal digital vault to organize, store, and find important information.VaultNote is a desktop application developed in Rust with the goal of offering a personal space to store and organize different types of content in a single place.The core idea is to allow users to keep notes, screenshots, photos, images, files, links, and other important content, organizing everything in a simple, local, and structured way.The project seeks to combine a modern interface with the security, performance, and reliability offered by the Rust ecosystem.📌 Table of ContentsAbout the projectObjectiveFeaturesProject statusTechnologiesArchitectureFolder structureRust installationHow to runMain dependenciesUsage examplesRoadmapFuture featuresProject principlesStatus📖 About the projectVaultNote is designed to be a desktop application that functions as a personal digital vault.Instead of scattering important information across different programs and computer folders, the goal is to centralize content within a single application.Users will eventually be able to:📝 Create and organize notes🖼️ Store images, screenshots, and photos📁 Import files🔗 Save important links⭐ Star content as favorites🔎 Search for content🗂️ Organize information by categories🗑️ Move items to the trashThe idea is for VaultNote to be simple for daily use while having an architecture prepared to grow gradually.🎯 ObjectiveThe main objective of VaultNote is to create a personal application where the user can:Save, organize, and find important information in a simple and centralized way.The project will be developed prioritizing:Code organizationModular architectureLocal storageEase of maintenanceExpansion capabilityClear interfaceComponent reuseSeparation of interface, logic, and storage✨ Features📝 NotesPlannedThe note-taking system should allow you to:Create notesEdit notesDelete notesSet titlesWrite contentAdd categoriesSearch notesView recent notesFavorite notesExample of a note:PlaintextTitle:
Ideas for the new project

Category:
Programming

Content:
Create a desktop application using Rust,
Iced, and SQLite.
🖼️ Images and ScreenshotsPlannedUsers will be able to:Import imagesStore screenshotsStore photosOrganize imagesSearch imagesView basic informationAssociate images with categoriesFavorite contentInformation that can be stored:PlaintextID
Name
Path
Type
Category
Creation date
Modification date
Initially, files will remain stored locally on the computer, while SQLite will be used to store their metadata.📁 FilesPlannedThe file system should allow you to:Import filesOrganize files by categoryView basic informationSearch filesFavorite filesMove files to the trashOpen files using the operating systemPossible types:PlaintextPDF
TXT
DOCX
ZIP
JSON
CSV
Other compatible formats
Initial compatibility will be defined as the project develops.🔗 LinksPlannedVaultNote may allow storing important links in the future.Example:PlaintextTitle:
Rust Documentation

URL:
https://www.rust-lang.org/

Category:
Programming
🗂️ OrganizationThe application will feature resources to facilitate content organization.PlannedCategory systemFavoritesList of recent itemsGlobal searchFiltersTrashFuture tag support🖥️ InterfaceThe interface will be built using Iced.The initial planned structure will look similar to:Plaintext
┌──────────────────────────────────────────────────────┐
│ SIDEBAR      │         CONTENT                       │
│              │                                       │
│ 🏠 Home      │         Dashboard / Current Screen    │
│ 📝 Notes     │                                       │
│ 🖼️ Images    │                                       │
│ 📁 Files     │                                       │
│ ⭐ Favorites │                                       │
│ 🗑️ Trash     │                                       │
│ ⚙️ Settings  │                                       │
│              │                                       │
└──────────────────────────────────────────────────────┘
Planned screensScreenDescriptionStatus🏠 HomeMain dashboard⏳ Planned📝 NotesNotes management⏳ Planned🖼️ ImagesImage and screenshot viewer⏳ Planned📁 FilesFile management⏳ Planned⭐ FavoritesContent marked as favorites⏳ Planned🗑️ TrashRemoved items⏳ Planned⚙️ SettingsApplication preferences⏳ PlannedScreen names, organization, and details may evolve during development.📊 Project statusVaultNote is currently in its initial development and architecture planning phase.To avoid confusion between ideas and existing features, the project uses the following statuses:StatusMeaning✅ CompletedFeature implemented and working🚧 In developmentFeature currently being built⏳ PlannedFeature defined, but not yet started💡 FutureIdea for future versionsCurrent situationAreaStatusProject planning🚧 In developmentInitial architecture🚧 In developmentGraphical interface⏳ PlannedScreen navigation⏳ PlannedNote system⏳ PlannedSQLite database⏳ PlannedFile system⏳ PlannedCategories⏳ PlannedSearch⏳ PlannedFavorites⏳ PlannedTrash⏳ PlannedImportant: features marked as planned should not be considered implemented.🦀 Technologies usedVaultNote will be developed primarily using:TechnologyFunctionRustMain languageIcedGraphical user interfaceSQLiteLocal databaseSQLxDatabase communicationTokioAsynchronous operations when neededOther libraries may be added as project requirements dictate.The priority will be to maintain an organized foundation and avoid unnecessary dependencies.🏗️ Project architectureThe project will be organized by separating responsibilities.The general idea will be:Plaintext┌──────────────────────────────┐
│            VIEWS             │
│                              │
│ Screens and visual elements  │
└───────────────┬──────────────┘
                │
                ▼
┌──────────────────────────────┐
│          APP / STATE         │
│                              │
│ Application state and flow   │
└───────────────┬──────────────┘
                │
                ▼
┌──────────────────────────────┐
│           SERVICES           │
│                              │
│ Business logic               │
└───────────────┬──────────────┘
                │
                ▼
┌──────────────────────────────┐
│           DATABASE           │
│                              │
│ SQLite + SQLx                │
└──────────────────────────────┘
Separation of responsibilitiesviews/Responsible for application screens.Examples:Plaintexthome.rs
notes.rs
images.rs
files.rs
favorites.rs
trash.rs
settings.rs
Views should focus mainly on building the interface.components/Contains reusable components.Examples:Plaintextsidebar
header
search
item_card
The goal is to avoid code duplication across different screens.models/Represents the data structures used by the application.Example:Rustpub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
}
Other models may represent:NotesFilesCategoriesFavoritesOther contentdatabase/Responsible for database connection and communication.Example responsibilities:PlaintextCreate connection
Initialize database
Execute queries
Create tables
Manage migrations in the future
services/Responsible for application logic.Examples:PlaintextCreate note
Edit note
Delete note
List files
Import file
Search content
Expected flowA simplified flow might work like this:PlaintextUser
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
This separation should facilitate project growth and maintenance.📁 Folder structureProposed initial structure:Plaintextvaultnote/
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
The structure may change as the project grows. The goal is to start with a clear organization without creating unnecessary complexity.🚀 Rust installationTo develop or run the project, Rust must be installed.The recommended way is to use the official installer available on the website:rustup.rsAfter installation, verify that everything is working:Bashrustc --version
Also check Cargo:Bashcargo --version
▶️ How to run the project1. Clone the repositoryWhen the repository is available:Bashgit clone <REPOSITORY_URL>
Enter the folder:Bashcd vaultnote
2. Run the projectUse:Bashcargo run
Cargo will be responsible for:Downloading the necessary dependenciesCompiling the projectRunning the application3. Compile without runningBashcargo build
For an optimized compilation:Bashcargo build --release
📦 Main dependenciesInitial dependencies should include libraries similar to these:Ini, TOML[dependencies]

iced = "..."
sqlx = { version = "...", features = ["sqlite"] }

tokio = {
    version = "...",
    features = ["full"]
}
Specific versions will be defined during project setup.Additional dependencies may be used for specific features, such as:File selectionPath manipulationDates and timesIdentifiersError handlingSerializationThe project should avoid adding libraries purely for convenience when simple features can be implemented or resolved using existing dependencies.📝 Usage flow exampleA possible flow for saving a note:Plaintext1. User opens VaultNote

2. Clicks on:
    + New note

3. Defines:
    Title
    Category
    Content

4. Clicks on:
    Save

5. The application:

    ├── Validates the data
    ├── Creates the note model
    ├── Calls the responsible service
    └── Saves the data to SQLite
Organization exampleA user might have a conceptual structure similar to:Plaintext📁 Programming
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
The exact organization format will be defined during data architecture development.🗺️ Development roadmapDevelopment will be divided into stages to allow gradual project evolution.🧱 Phase 1 — BaseObjectiveCreate the minimal application structure.Planned[ ] Create Rust project[ ] Configure Iced[ ] Create main window[ ] Create initial module structure[ ] Create App[ ] Create basic message system[ ] Create state management[ ] Create basic screen navigation system🎨 Phase 2 — InterfaceObjectiveCreate VaultNote's main visual structure.Planned[ ] Create sidebar[ ] Create visual navigation[ ] Create dashboard[ ] Create notes screen[ ] Create initial images screen[ ] Create initial files screen[ ] Create reusable components[ ] Create header[ ] Create button to add content[ ] Improve layout and spacing🗄️ Phase 3 — DatabaseObjectiveImplement local storage using SQLite.Planned[ ] Configure SQLite[ ] Configure SQLx[ ] Create database connection[ ] Create initial table structure[ ] Create notes model[ ] Save notes[ ] List notes[ ] Edit notes[ ] Delete notes📁 Phase 4 — FilesObjectiveAllow file storage and management.Planned[ ] Create file import system[ ] Create storage folder[ ] Create folder for images[ ] Create folder for files[ ] Save metadata to SQLite[ ] List files[ ] Display basic information[ ] Implement file opening🗂️ Phase 5 — OrganizationObjectiveAdd features to facilitate navigation between content.Planned[ ] Create categories[ ] Implement favorites[ ] Create search[ ] Create global search[ ] Implement trash[ ] Create filters[ ] Create recent items list🚀 Phase 6 — Future improvementsObjectiveExpand the application after the main foundation is stable.Planned[ ] Tag system[ ] Data backup[ ] Data export[ ] Light theme[ ] Dark theme[ ] Optional encryption[ ] Security improvements[ ] Future cross-device synchronization🔮 Future featuresThe features below are ideas for future versions and are not part of the current implementation.🏷️ TagsAllow multiple tags for the same content.Example:Plaintext#rust
#programming
#studies
#project
💾 BackupPossibility to create data backups.Possible formats:PlaintextDatabase backup
File backup
Full backup
📤 ExportPossibility to export content.Future examples:PlaintextExport notes
Export data
Export categories
Create backup package
🎨 ThemesFuture support for:Light themeDark themeOperating system preference matching🔐 Optional encryptionA future possibility is to allow certain data to be protected by additional security mechanisms.This feature must be carefully planned before implementation.🔄 Cross-device synchronizationA long-term possibility is to allow synchronization between different devices.Conceptual example:PlaintextDesktop
    │
    ▼
Sync system
    │
    ├── Laptop
    │
    └── Other device
This feature is not part of the first version of the project.🧠 Project principlesVaultNote development will follow a few important principles.SimplicityStart small.PlaintextFirst:
Working application

Then:
Features

Then:
Improvements

Then:
Expansions
OrganizationCode must have well-separated responsibilities.Avoid:PlaintextInterface
+
Database
+
Logic
+
Files
all mixed into the same file.Prefer:Plaintextviews/
services/
models/
database/
components/
Gradual evolutionNew features should be added when the foundation is working.The goal is not to implement all ideas immediately.Local storageThe first version should prioritize:PlaintextLocal application
+
SQLite
+
Local files
Online synchronization features can be evaluated later.🤝 ContributionThe project is still in its initial phase.When the repository is structured for collaboration, this section will include information on:How to open an issueHow to suggest featuresHow to contribute codeCommit conventionsCode standardsPull requests📄 LicenseThe project license is yet to be defined.Until a license is added, the repository must clearly state the terms of use and distribution chosen by the project.📌 Current status🚧 In developmentVaultNote is currently in the initial phase of planning and building the project foundation.Next stepsPlaintext1. Create Rust project
        ↓
2. Configure Iced
        ↓
3. Create main window
        ↓
4. Organize modules
        ↓
5. Create App, State, and Message
        ↓
6. Implement basic navigation
        ↓
7. Build initial interface
🗄️ VaultNoteA place to keep what matters.The goal is to build an organized, modular desktop application prepared to evolve, starting with a simple foundation in Rust, Iced, and SQLite.Status: 🚧 In developmentVersion: Not yet definedInitial platform: DesktopMain language: Rust 🦀
