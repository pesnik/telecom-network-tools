import React, { createContext, useContext, useState, ReactNode } from "react";

interface FileContextType {
  selectedFile: string | null;
  setSelectedFile: React.Dispatch<React.SetStateAction<string | null>>;
}

const FileContext = createContext<FileContextType | undefined>(undefined);

export const useFileContext = () => {
  const context = useContext(FileContext);
  if (!context) {
    throw new Error("useFileContext must be used within a FileProvider");
  }
  return context;
};

export const FileProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  return (
    <FileContext.Provider value={{ selectedFile, setSelectedFile }}>
      {children}
    </FileContext.Provider>
  );
};
