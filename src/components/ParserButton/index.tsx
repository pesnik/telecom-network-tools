import React from "react";
import Button from "@mui/material/Button";
import { useFileContext } from "contexts/FileContext";
import { invoke } from "@tauri-apps/api/tauri";

const FileParserButton: React.FC = () => {
  const { selectedFile, setSelectedFile } = useFileContext();

  const sendFileForParsing = async () => {
    if (!selectedFile) return;

    try {
      const res = await invoke("greet", { name: selectedFile });
      setSelectedFile(res as string);
    } catch (error) {
      console.error("Error parsing file:", error);
    }
  };

  return (
    <Button
      disabled={!selectedFile}
      variant="contained"
      onClick={sendFileForParsing}
    >
      Parse Dependency
    </Button>
  );
};

export default FileParserButton;
