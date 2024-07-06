import * as React from "react";
import { CssBaseline } from "@mui/material";
import FileUploader from "components/FileUpload";
import Container from "@mui/material/Container";
import Stack from "@mui/material/Stack";
import ParserButton from "components/ParserButton";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { FileProvider } from "contexts/FileContext";

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
});

const App: React.FC = () => {
  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <FileProvider>
        <Container maxWidth="sm">
          <Stack spacing={2} sx={{ mt: "10%" }}>
            <FileUploader />
            <ParserButton />
          </Stack>
        </Container>
      </FileProvider>
    </ThemeProvider>
  );
};

export default App;
