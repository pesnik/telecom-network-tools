import { styled } from "@mui/material/styles";
import { useFileContext } from "contexts/FileContext";

import Divider from "@mui/material/Divider";
import IconButton from "@mui/material/IconButton";
import Paper from "@mui/material/Paper";
import DeleteIcon from "@mui/icons-material/Delete";
import InsertDriveFileIcon from "@mui/icons-material/InsertDriveFile";

const ContentContainer = styled("div")({
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  justifyContent: "center",
  marginTop: "20px",
});

const SelectedActualFile = (): React.ReactNode => {
  const { selectedFile, setSelectedFile } = useFileContext();

  const handleClearFile = () => {
    setSelectedFile(null);
  };

  return (
    <ContentContainer>
      <Divider />
      <Paper
        variant="outlined"
        sx={{
          padding: "10px",
          display: "flex",
          gap: "5px",
          textAlign: "center",
          alignItems: "center",
        }}
      >
        <InsertDriveFileIcon />
        {selectedFile}
        <IconButton
          color="error"
          aria-label="delete file"
          onClick={handleClearFile}
        >
          <DeleteIcon />
        </IconButton>
      </Paper>
    </ContentContainer>
  );
};

export default SelectedActualFile;
