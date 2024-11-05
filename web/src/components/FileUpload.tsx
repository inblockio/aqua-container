import axios from 'axios';
import { Component, createEffect, createSignal } from 'solid-js';
import { appState, setAppState } from '../store/store';
import { ApiFileInfo } from '../models/FileInfo';
import { useNavigate } from '@solidjs/router';
import { readJsonFile } from '../util';

type FileUploadProps = {
  onFilesSelected?: (files: File[]) => void;
};

const FileUpload: Component<FileUploadProps> = (props: FileUploadProps) => {

  const [isDragging, setIsDragging] = createSignal(false);
  const [files, setFiles] = createSignal<File[]>([]);

  const [uploadedFilesIndexes, setUploadedFilesIndexes] = createSignal<number[]>([])

  const navigate = useNavigate();

  const handleDragEnter = (event: DragEvent) => {
    event.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = (event: DragEvent) => {
    event.preventDefault();
    setIsDragging(false);
  };

  const handleDrop = (event: DragEvent) => {
    event.preventDefault();
    setIsDragging(false);

    if (event.dataTransfer) {
      const droppedFiles = Array.from(event.dataTransfer.files);
      setFiles(droppedFiles);
      props.onFilesSelected?.(droppedFiles);
    }
  };

  const handleFileSelect = (event: Event & { target: HTMLInputElement }) => {
    const selectedFiles = event.target.files ? Array.from(event.target.files) : [];
    setFiles(selectedFiles);
    props.onFilesSelected?.(selectedFiles);
  };

  const isJSONFile = (fileName: string) => {
    return fileName.trim().toLowerCase().endsWith('.json');
  }

  const uploadAquaJsonFile = async (fileIndex: number) => {
    const file = files()[fileIndex];
    if (!file) {
      alert("File not found")
      return;
    }

    const formData = new FormData();
    formData.append('file', file);
    formData.append('account', "example");

    try {
      const response = await axios.post('http://127.0.0.1:3600/explorer_aqua_file_upload', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
          "metamask_address": appState.metaMaskAddress
        },
      });

      const res = response.data

      let logs: Array<string> = res.logs
      logs.forEach((item) => {
        console.log("**>" + item + "\n.")
      })

      console.log("Upload res: ", res)
      // Assuming the API returns an array of FileInfo objects
      const file: ApiFileInfo = {
        id: res.file.id,
        name: res.file.name,
        extension: res.file.extension,
        page_data: res.file.page_data,
        mode: '',
        owner: ''
      };

      setAppState("filesFromApi", [...appState.filesFromApi, file])
      setUploadedFilesIndexes(value => [...value, fileIndex])
      return;
    } catch (error) {
      console.error('Error uploading file:', error);
      alert("Error uploading file")
    }
  };

  const uploadFile = async (fileIndex: number) => {
    const file = files()[fileIndex]
    if (!file) {
      alert('No file selected');
      return;
    }

    const formData = new FormData();
    formData.append('file', file);
    formData.append('account', "example");

    try {
      const response = await axios.post('http://127.0.0.1:3600/explorer_file_upload', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
          "metamask_address": appState.metaMaskAddress
        },
      });

      const res = response.data

      let logs: Array<string> = res.logs
      logs.forEach((item) => {
        console.log("**>" + item + "\n.")
      })


      // Assuming the API returns an array of FileInfo objects
      const file: ApiFileInfo = {
        id: res.file.id,
        name: res.file.name,
        extension: res.file.extension,
        page_data: res.file.page_data,
        mode: res.file.mode,
        owner: res.file.owner
      };

      setAppState("filesFromApi", [...appState.filesFromApi, file])
      setUploadedFilesIndexes(value => [...value, fileIndex])
      return;
    } catch (error) {
      console.error('Error uploading file:', error);
      alert('Failed to upload file');
    }
  };

  const handleVerifyAquaJsonFile = (fileIndex: number) => {
    const file = files()[fileIndex];

    readJsonFile(file)
      .then((jsonData) => {
        const hashChain: ApiFileInfo = {
          id: 0,
          name: '',
          extension: '',
          page_data: JSON.stringify(jsonData),
          mode: '',
          owner: ''
        }
        // const hashChainString = JSON.stringify(hashChain)
        // console.log("JSON data:", hashChain);
        setAppState("selectedFileFromApi", hashChain);
        navigate("/details");
        // Handle the JSON data here
      })
      .catch((error) => {
        console.error("Error reading JSON file:", error.message);
        // Handle the error here
      });
  };

  const displayUploadButtons = (fileIndex: number) => {
    let file = files()[fileIndex]
    if (!file) {
      return 'none'
    }
    let isJsonFile = isJSONFile(file.name)
    let isInUploaded = uploadedFilesIndexes().includes(fileIndex)
    if (isJsonFile && !isInUploaded) {
      return 'block'
    } else {
      return 'none'
    }
  }

  // createEffect(() => {
  //   console.log(uploadedFilesIndexes())
  // }, [uploadedFilesIndexes()])

  return (
    <>
      <div
        onDragEnter={handleDragEnter}
        onDragOver={(e) => e.preventDefault()}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        style={{
          border: '2px dashed #cccccc',
          "border-radius": '8px',
          padding: '20px',
          "text-align": 'center',
          cursor: 'pointer',
          "background-color": isDragging() ? '#e3f2fd' : '#fafafa',
        }}
        class='mb-5'
      >
        {/* Hidden file input */}
        <input
          type="file"
          multiple
          // accept=".pdf, image/*"
          onChange={handleFileSelect}
          style={{ display: 'none' }}
          id="file-upload-input"
        />

        {/* Label to trigger file input */}
        <label
          for="file-upload-input"
          style={{ cursor: 'pointer' }}
          onClick={() => document.getElementById("file-upload-input")?.click()}
          class='text-2xl'
        >
          {isDragging() ? (
            <p>Drop the files here ...</p>
          ) : (
            <p>Drag & drop some files here, or click to select files</p>
          )}
        </label>
        <p class='text-2xl'>
          {`You have selected ${files().length} Files`}
        </p>
        {/* File Previews */}
        <ul>
          {/* {files().map((file) => (
            <li key={file.name}>{file.name}</li>
          ))} */}
        </ul>
      </div>
      <div class='mt-5 mb-5' style={{ display: files().length === 0 ? 'none' : 'block' }}>
        <div class="card rounded-md">
          <div class="overflow-x-auto">
            <div class="min-w-full inline-block align-middle">
              <div class="border rounded-lg shadow overflow-hidden dark:border-gray-700 dark:shadow-gray-900">
                <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                  <thead>
                    <tr class="divide-x divide-gray-200 dark:divide-gray-700">
                      <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">File Name</th>
                      <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Size</th>
                      <th scope="col" class="px-6 py-3 text-end text-xs font-medium text-gray-500 uppercase">Action</th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                    {files().map((file, i: number) => (
                      <tr>
                        <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-800 dark:text-gray-200">{file.name}</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-800 dark:text-gray-200">{`${bytesToMB(file.size)} MB`}</td>
                        <td class="px-6 py-4 whitespace-nowrap text-end text-sm font-medium">
                          <div style={{
                            display: 'flex',
                            "column-gap": '10px',
                            "justify-content": 'flex-end'
                          }}>
                            <button type="button" class="btn btn-sm border border-primary text-primary hover:bg-primary hover:text-white" style={{
                              display: uploadedFilesIndexes().includes(i) ? 'none' : 'block'
                            }} onclick={() => uploadFile(i)}>Upload</button>
                            <button type="button" style={{ display: isJSONFile(file.name) ? 'block' : 'none' }} class="btn btn-sm border border-primary text-primary hover:bg-primary hover:text-white" onclick={() => handleVerifyAquaJsonFile(i)}>Verify Aqua File</button>
                            <button type="button" style={{ display: displayUploadButtons(i) }} class="btn btn-sm border border-primary text-primary hover:bg-primary hover:text-white" onclick={() => uploadAquaJsonFile(i)}
                            >Import Aqua File</button>
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>

      </div>
    </>
  );
};

function bytesToMB(bytes: number): string {
  const megabytes = bytes / (1024 * 1024);
  return megabytes.toFixed(3); // Rounds to 3 decimal places and returns as a string
}

export default FileUpload;
