import { useState, useEffect } from "react";
import {
  ThemeProvider,
  createTheme,
  CssBaseline,
  Container,
  Paper,
  Typography,
  TextField,
  Box,
  IconButton,
  List,
  ListItem,
  ListItemText,
  Divider,
} from "@mui/material";
import { Delete } from "@mui/icons-material";

interface TableEntry {
  video_id: string;
  title: string;
  channel_id: string;
  published: string;
}

// Create dark theme
const darkTheme = createTheme({
  palette: {
    mode: "dark",
    primary: {
      main: "#ffffff",
    },
    background: {
      default: "#000000",
      paper: "#121212",
    },
    text: {
      primary: "#ffffff",
    },
  },
});

export default function Table() {
  const [entries, setEntries] = useState<TableEntry[]>([]);
  const [searchQuery, setSearchQuery] = useState("");

  async function refreshEntries() {
    try {
      const response = await fetch(
        "http://127.0.0.1:8080/api/unwatched_videos"
      );
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }

      setEntries(await response.json());
    } catch (error) {
      console.error(error);
    }
  }

  // Request data to populate the table with.
  useEffect(() => {
    refreshEntries();
  }, []);

  async function onClickDelete(entryId: string) {
    const serverKey = "todo";

    const response = await fetch(
      "http://127.0.0.1:8080/api/set_video_watched",
      {
        method: "POST",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          server_key: serverKey,
          video_id: entryId,
        }),
      }
    );

    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    refreshEntries();
  }

  function onClickWatch(entryId: string) {
    window.open("https://www.youtube.com/watch?v=" + entryId);
    onClickDelete(entryId);
  }

  // Sort entries by date published (oldest first)
  const sortedEntries = [...entries].sort((a: TableEntry, b: TableEntry) => {
    return new Date(a.published).getTime() - new Date(b.published).getTime();
  });

  // Filter entries based on search query
  const filteredEntries = sortedEntries.filter((entry) => {
    const query = searchQuery.toLowerCase();
    return (
      entry.channel_id.toLowerCase().includes(query) ||
      entry.title.toLowerCase().includes(query) ||
      entry.published.toLowerCase().includes(query)
    );
  });

  // Don't render if there's no data
  if (entries.length === 0) {
    return (
      <ThemeProvider theme={darkTheme}>
        <CssBaseline />
        <Container maxWidth="md" sx={{ py: 4 }}>
          <Typography variant="h6" align="center">
            Loading...
          </Typography>
        </Container>
      </ThemeProvider>
    );
  }

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <Container maxWidth="md" sx={{ py: 4 }}>
        <Typography variant="h4" component="h1" gutterBottom align="center">
          YouTube Videos ({entries.length})
        </Typography>
        
        <Box sx={{ mb: 3 }}>
          <TextField
            fullWidth
            variant="outlined"
            placeholder="Search by channel ID, title, or date..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            sx={{ mb: 2 }}
          />
        </Box>

        <Paper elevation={3}>
          <List>
            {filteredEntries.map((entry, index) => (
              <Box key={entry.video_id}>
                <ListItem
                  sx={{
                    cursor: "pointer",
                    "&:hover": {
                      backgroundColor: "action.hover",
                    },
                    px: 2,
                    py: 2,
                    display: "flex",
                    alignItems: "flex-start",
                  }}
                  onClick={() => onClickWatch(entry.video_id)}
                >
                  <Box sx={{ flexGrow: 1 }}>
                    <ListItemText
                      primary={
                        <Typography
                          variant="h6"
                          component="h2"
                          sx={{ fontWeight: "bold", mb: 1 }}
                        >
                          {entry.channel_id}
                        </Typography>
                      }
                      secondary={
                        <Box sx={{ display: "flex", alignItems: "center", gap: 2 }}>
                          <Typography
                            variant="body2"
                            color="text.secondary"
                            sx={{ flexShrink: 0 }}
                          >
                            {entry.published}
                          </Typography>
                          <Typography variant="body1" color="text.primary">
                            {entry.title}
                          </Typography>
                        </Box>
                      }
                    />
                  </Box>
                  <IconButton
                    edge="end"
                    aria-label="delete"
                    onClick={(e) => {
                      e.stopPropagation();
                      onClickDelete(entry.video_id);
                    }}
                    sx={{ ml: 2 }}
                  >
                    <Delete />
                  </IconButton>
                </ListItem>
                {index < filteredEntries.length - 1 && <Divider />}
              </Box>
            ))}
          </List>
        </Paper>

        {filteredEntries.length === 0 && searchQuery && (
          <Box sx={{ mt: 3, textAlign: "center" }}>
            <Typography variant="body1" color="text.secondary">
              No entries found matching "{searchQuery}"
            </Typography>
          </Box>
        )}
      </Container>
    </ThemeProvider>
  );
}
