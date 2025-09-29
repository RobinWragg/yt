# GitHub Copilot Instructions for YouTube Video Tracker

## Project Overview

This is a YouTube video tracking application that helps users monitor and manage videos from their subscribed YouTube channels. The application consists of a Rust backend API and a React TypeScript frontend.

## Architecture

### Backend (Rust)
- **Framework**: Actix-web for HTTP server and REST API
- **Database**: PostgreSQL with SQLx for async database operations
- **Key Components**:
  - `main.rs`: HTTP server setup, API route handlers
  - `database.rs`: Database operations and SQL queries
  - `crawler.rs`: YouTube channel scraping and video extraction
- **API Endpoints**:
  - `GET /api/unwatched_videos`: Returns JSON array of unwatched videos
  - `POST /api/set_video_watched`: Marks a video as watched
  - `GET /api/all_channel_ids`: Returns all tracked channel IDs

### Frontend (React + TypeScript)
- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite
- **UI Library**: Material-UI (MUI) v5 with dark theme
- **Routing**: React Router v6
- **Key Components**:
  - `main.tsx`: Route configuration and app initialization
  - `yt.tsx`: Main video list component with search and interaction
  - `root.tsx`: Landing page component

### Database Schema
- **videos table**: video_id (PK), channel_id, title, published_date, watched (boolean)
- **channels table**: channel_id (PK), additional metadata

## Code Style and Patterns

### Rust Backend
- Use `async fn` for all API handlers
- Error handling with `Result<T, Box<dyn Error>>`
- Database operations use SQLx with prepared statements
- Follow Rust naming conventions (snake_case for functions/variables)
- Use `serde` for JSON serialization/deserialization
- Handle database errors gracefully with proper HTTP status codes

### TypeScript Frontend
- Use functional components with hooks (useState, useEffect)
- TypeScript interfaces for all data structures
- Material-UI components with sx prop for styling
- Async/await for API calls with proper error handling
- Use React Router for navigation
- Dark theme implementation using MUI's ThemeProvider

### Key Data Structures
```typescript
interface TableEntry {
  video_id: string;
  title: string;
  channel_id: string;
  published: string;
}
```

```rust
struct VideoDetails {
  video_id: String,
  channel_id: String,
  title: String,
  date: DateTime<Utc>,
}
```

## Development Workflow

### Backend Development
- Use `cargo check` for fast compilation checking
- Use `cargo run` to start the development server (binds to 127.0.0.1:8080)
- Database connection requires PostgreSQL running locally
- Static files served from `../frontend/dist/`

### Frontend Development
- Use `npm run dev` for development server with hot reload
- Use `npm run build` to create production build in `dist/` folder
- Use `npm run lint` for ESLint checking
- Build output is consumed by the Rust backend

### Build Process
The `run.sh` script automates the complete build:
1. Build frontend with `npm run build`
2. Build and run backend with `cargo run`

## API Integration Patterns

### Frontend to Backend Communication
- Base URL: `http://127.0.0.1:8080`
- All API calls use fetch() with proper headers
- POST requests include JSON body with Content-Type header
- Error handling with response.ok checking

### Video Management Flow
1. Load videos: `GET /api/unwatched_videos`
2. Mark watched: `POST /api/set_video_watched` with video_id
3. Open video: Direct link to `https://www.youtube.com/watch?v={video_id}`

## Component Patterns

### Material-UI Usage
- Use ThemeProvider with dark theme for consistent styling
- Container components for layout structure
- Paper components for elevated content areas
- Typography components with proper variant hierarchy
- IconButton components for interactive elements

### State Management
- Use useState for component-level state
- Use useEffect for data fetching on component mount
- Implement refresh functions for updating data after mutations

## File Organization

```
├── backend/
│   ├── src/
│   │   ├── main.rs          # Server and API routes
│   │   ├── database.rs      # Database operations
│   │   └── crawler.rs       # YouTube scraping logic
│   └── Cargo.toml          # Rust dependencies
├── frontend/
│   ├── src/
│   │   ├── main.tsx        # Router setup
│   │   ├── yt.tsx          # Main video list component
│   │   └── root.tsx        # Landing page
│   ├── package.json        # Node dependencies
│   └── vite.config.ts      # Vite configuration
└── run.sh                  # Build and run script
```

## Common Patterns to Follow

1. **Database Operations**: Always use parameterized queries with SQLx
2. **Error Handling**: Return proper HTTP status codes (200, 404, 500)
3. **Component Structure**: Keep components focused and reusable
4. **State Updates**: Use functional state updates for arrays and objects
5. **API Calls**: Always handle both success and error cases
6. **Styling**: Use Material-UI's sx prop for component-specific styles
7. **Type Safety**: Define TypeScript interfaces for all data structures

## Testing Considerations

- Backend has test files in `backend/test_files/`
- Use `cargo test` for running Rust tests
- Frontend uses ESLint for code quality checking
- Manual testing involves running the full application stack

This is a full-stack application where the Rust backend serves both API endpoints and the built frontend assets, creating a single deployable unit.