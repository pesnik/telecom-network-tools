import * as React from "react";
import { CssBaseline, Box, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Toolbar, AppBar, Typography, Divider } from "@mui/material";
import Container from "@mui/material/Container";
import Stack from "@mui/material/Stack";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { FileProvider } from "contexts/FileContext";
import FileUploader from "components/FileUpload";
import ParserButton from "components/ParserButton";
import FileUploadIcon from '@mui/icons-material/FileUpload';
import DeviceHubIcon from '@mui/icons-material/DeviceHub';
import BugReportIcon from '@mui/icons-material/BugReport';
import NetworkCheckIcon from '@mui/icons-material/NetworkCheck';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
});

const drawerWidth = 240;

const App: React.FC = () => {
  const [selectedTool, setSelectedTool] = React.useState<string>("excelParser");
  const [macAddress, setMacAddress] = React.useState<string>("");
  const [neName, setNeName] = React.useState<string>("");

  const handleToolSelect = (tool: string) => {
    setSelectedTool(tool);
  };

  const renderContent = () => {
    switch (selectedTool) {
      case "excelParser":
        return (
          <Stack spacing={2} sx={{ width: '100%' }}>
            <FileUploader />
            <ParserButton />
          </Stack>
        );
      case "macPathfinder":
        return (
          <Stack spacing={2} sx={{ width: '100%' }}>
            <Typography variant="h6">MAC Address Pathfinder</Typography>
            <Typography variant="body2">
              Visualize all possible transmission paths for a given MAC address
            </Typography>
            <TextField
              label="MAC Address"
              placeholder="Enter MAC address (e.g., 00:1A:2B:3C:4D:5E)"
              fullWidth
              value={macAddress}
              onChange={(e) => setMacAddress(e.target.value)}
            />
            <Button 
              variant="contained" 
              startIcon={<DeviceHubIcon />}
              disabled={!macAddress}
            >
              Generate Path Map
            </Button>
            <Box sx={{ mt: 2, height: 400, border: '1px dashed grey', borderRadius: 1, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
              <Typography variant="body2" color="text.secondary">Path visualization will appear here</Typography>
            </Box>
          </Stack>
        );
      case "neDebugger":
        return (
          <Stack spacing={2} sx={{ width: '100%' }}>
            <Typography variant="h6">Network Element Debugger</Typography>
            <Typography variant="body2">
              Debug transmission paths for network elements
            </Typography>
            <TextField
              label="Network Element Name"
              placeholder="Enter NE name"
              fullWidth
              value={neName}
              onChange={(e) => setNeName(e.target.value)}
            />
            <Button 
              variant="contained" 
              startIcon={<BugReportIcon />}
              disabled={!neName}
            >
              Analyze Paths
            </Button>
            <Box sx={{ mt: 2, height: 400, border: '1px dashed grey', borderRadius: 1, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
              <Typography variant="body2" color="text.secondary">Condition met and unmet paths will appear here</Typography>
            </Box>
          </Stack>
        );
      default:
        return <Typography>Select a tool from the sidebar</Typography>;
    }
  };

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <FileProvider>
        <Box sx={{ display: 'flex' }}>
          <AppBar 
            position="fixed" 
            sx={{ width: `calc(100% - ${drawerWidth}px)`, ml: `${drawerWidth}px` }}
          >
            <Toolbar>
              <Typography variant="h6" noWrap component="div">
                Telecom Network Tools
              </Typography>
            </Toolbar>
          </AppBar>
          
          <Drawer
            sx={{
              width: drawerWidth,
              flexShrink: 0,
              '& .MuiDrawer-paper': {
                width: drawerWidth,
                boxSizing: 'border-box',
              },
            }}
            variant="permanent"
            anchor="left"
          >
            <Toolbar>
              <Typography variant="h6" align="center">Tools</Typography>
            </Toolbar>
            <Divider />
            <List>
              <ListItem disablePadding>
                <ListItemButton 
                  selected={selectedTool === "excelParser"}
                  onClick={() => handleToolSelect("excelParser")}
                >
                  <ListItemIcon>
                    <FileUploadIcon />
                  </ListItemIcon>
                  <ListItemText primary="Excel Parser" />
                </ListItemButton>
              </ListItem>
              
              <ListItem disablePadding>
                <ListItemButton
                  selected={selectedTool === "macPathfinder"}
                  onClick={() => handleToolSelect("macPathfinder")}
                >
                  <ListItemIcon>
                    <NetworkCheckIcon />
                  </ListItemIcon>
                  <ListItemText primary="MAC Pathfinder" />
                </ListItemButton>
              </ListItem>
              
              <ListItem disablePadding>
                <ListItemButton
                  selected={selectedTool === "neDebugger"}
                  onClick={() => handleToolSelect("neDebugger")}
                >
                  <ListItemIcon>
                    <BugReportIcon />
                  </ListItemIcon>
                  <ListItemText primary="NE Debugger" />
                </ListItemButton>
              </ListItem>
            </List>
          </Drawer>
          
          <Box
            component="main"
            sx={{ flexGrow: 1, p: 3, width: { sm: `calc(100% - ${drawerWidth}px)` } }}
          >
            <Toolbar /> {/* This creates space below the AppBar */}
            <Container>
              {renderContent()}
            </Container>
          </Box>
        </Box>
      </FileProvider>
    </ThemeProvider>
  );
};

export default App;
