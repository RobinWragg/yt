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
  Button,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
} from "@mui/material";
import { Delete, Add } from "@mui/icons-material";

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
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [channelId, setChannelId] = useState("");
  const [channels, setChannels] = useState<string[]>([]);

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

  async function refreshChannels() {
    try {
      const response = await fetch(
        "http://127.0.0.1:8080/api/all_channel_ids"
      );
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }

      setChannels(await response.json());
    } catch (error) {
      console.error(error);
    }
  }

  // Request data to populate the table with.
  useEffect(() => {
    refreshEntries();
    refreshChannels();
  }, []);

  async function onClickDelete(entryId: string) {
    const response = await fetch(
      "http://127.0.0.1:8080/api/set_video_watched",
      {
        method: "POST",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
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

  async function onAddChannel() {
    if (!channelId.trim()) {
      return;
    }

    try {
      const response = await fetch("http://127.0.0.1:8080/api/insert_channel", {
        method: "POST",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          channel_id: channelId.trim(),
        }),
      });

      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }

      // Close modal and reset form
      setIsModalOpen(false);
      setChannelId("");
      
      // Refresh channels list to show new channel
      refreshChannels();
    } catch (error) {
      console.error("Failed to add channel:", error);
      // You could add user-facing error handling here
    }
  }

  async function onDeleteChannel(channelIdToDelete: string) {
    try {
      const response = await fetch("http://127.0.0.1:8080/api/delete_channel", {
        method: "POST",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          channel_id: channelIdToDelete,
        }),
      });

      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`);
      }

      // Refresh both channels and videos after deletion
      refreshChannels();
      refreshEntries();
    } catch (error) {
      console.error("Failed to delete channel:", error);
    }
  }

  function handleModalClose() {
    setIsModalOpen(false);
    setChannelId("");
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

        {/* Channels Section */}
        {channels.length > 0 && (
          <Paper elevation={3} sx={{ mb: 3, p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Channels ({channels.length})
            </Typography>
            <Box sx={{ display: "flex", flexWrap: "wrap", gap: 1 }}>
              {channels.map((channel) => (
                <Box
                  key={channel}
                  sx={{
                    display: "flex",
                    alignItems: "center",
                    gap: 1,
                    px: 2,
                    py: 1,
                    backgroundColor: "action.hover",
                    borderRadius: 1,
                  }}
                >
                  <Typography variant="body2">{channel}</Typography>
                  <IconButton
                    size="small"
                    aria-label="delete channel"
                    onClick={() => onDeleteChannel(channel)}
                    sx={{ ml: 1 }}
                  >
                    <Delete fontSize="small" />
                  </IconButton>
                </Box>
              ))}
            </Box>
          </Paper>
        )}
        
        <Box sx={{ mb: 3, display: "flex", gap: 2, alignItems: "center" }}>
          <TextField
            fullWidth
            variant="outlined"
            placeholder="Search by channel ID, title, or date..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          <Button
            variant="contained"
            startIcon={<Add />}
            onClick={() => setIsModalOpen(true)}
            sx={{ 
              whiteSpace: "nowrap",
              minWidth: "auto",
              px: 2
            }}
          >
            Add Channel
          </Button>
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
                    alignItems: "center",
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

        {/* Add Channel Modal */}
        <Dialog 
          open={isModalOpen} 
          onClose={handleModalClose}
          maxWidth="sm"
          fullWidth
        >
          <DialogTitle>Add New Channel</DialogTitle>
          <DialogContent>
            <TextField
              autoFocus
              margin="dense"
              label="Channel ID"
              placeholder="Enter YouTube channel ID (e.g., UCxxxxxxxxxxxxxxxxx)"
              fullWidth
              variant="outlined"
              value={channelId}
              onChange={(e) => setChannelId(e.target.value)}
              sx={{ mt: 2 }}
            />
          </DialogContent>
          <DialogActions>
            <Button onClick={handleModalClose}>Cancel</Button>
            <Button 
              onClick={onAddChannel} 
              variant="contained"
              disabled={!channelId.trim()}
            >
              Add Channel
            </Button>
          </DialogActions>
        </Dialog>
      </Container>
    </ThemeProvider>
  );
}
