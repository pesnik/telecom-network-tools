import React from "react";
import { styled } from "@mui/material/styles";
import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import CardHeader from "@mui/material/CardHeader";
import FileUploadButton from "./FileUploadButton";
import Divider from "@mui/material/Divider";
import Paper from "@mui/material/Paper";
import InsertDriveFileIcon from "@mui/icons-material/InsertDriveFile";
import IconButton from "@mui/material/IconButton";
import DeleteIcon from "@mui/icons-material/Delete";
import { useFileContext } from "contexts/FileContext";

const ContentContainer = styled("div")({
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  justifyContent: "center",
  marginTop: "20px",
});

const FileUploader: React.FC = () => {
  const { selectedFile, setSelectedFile } = useFileContext();

  const handleFileSelect = (file: null | string) => {
    setSelectedFile(file);
  };

  const handleClearFile = () => {
    setSelectedFile(null);
  };

  return (
    <Card
      sx={{
        minWidth: 275,
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
      }}
    >
      <CardHeader
        title="Transmission Path Dependency Checker"
        sx={{ alignSelf: "center" }}
      />
      <CardContent>
        {selectedFile ? (
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
        ) : (
          <FileUploadButton onFileSelect={handleFileSelect} />
        )}
      </CardContent>
      <CardActions></CardActions>
    </Card>
  );
};

export default FileUploader;
