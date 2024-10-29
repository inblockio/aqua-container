import { Component, createSignal } from 'solid-js';


interface CopyableTextProps {
    copyText : string
    displayText : string
    onCopyMessage: string
}

export const CopyableText: Component<CopyableTextProps> = (props) => {

    const [isCopied, setIsCopied] = createSignal(false);

    const handleCopy = async () => {
        await navigator.clipboard.writeText(props.copyText);
        setIsCopied(true);
        setTimeout(() => setIsCopied(false), 2000); // Hide after 2 seconds
    };

    return (
        <div class="flex items-center text-gray-600 dark:text-gray-400 mt-5" style="font-family: 'monospace'">
            <span>{props.displayText}</span>
            <span class="m-5 p-3 flex items-center cursor-pointer" onClick={handleCopy}>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    fill="currentColor"
                    class="bi bi-copy ml-2"
                    viewBox="0 0 16 16"
                >
                    <path fill-rule="evenodd" d="M4 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm2-1a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1zM2 5a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1h1v1a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h1v1z" />
                </svg>
            </span>
            {isCopied() && (
                <span class="ml-2 bg-success/5 text-xsm font-small text-success hover:text-white hover:bg-success">
                    {props.onCopyMessage || 'Copied!'}
                </span>
            )}
        </div>
    );
};
