<?php

declare(strict_types=1);

// Stubs for lopdf

namespace LoPdf {
    use LoPdf\Exception\LoadException;
    use LoPdf\Exception\SaveException;

    class Document {
        /**
         * Opens a PDF file from filename/URI (supports stream wrappers).
         * 
         * @throws LoadException if file cannot be opened or there's an error while loading the file.
         */
        public static function open(string $path): self { }

        /**
         * Loads a PDF file from memory.
         *
         * @throws LoadException if there's an error while loading the file.
         */
        public static function load(string $contents): self { }

        /**
         * Saves the PDF file.
         * If $path is null, the PDF file is returned as string.
         *
         * @param string|null $path
         *
         * @throws SaveException if there's an error while saving the PDF file.
         */
        public function save(string|null $path = null): string|null { }

        /**
         * Compress the PDF file.
         */
        public function compress() { }

        /**
         * Decompress the PDF file.
         */
        public function decompress() { }

        /**
         * Check whether the PDF file is encrypted.
         */
        public function isEncrypted(): bool { }

        /**
         * Returns the number of pages in the PDF file.
         */
        public function countPages(): int { }
    }
}

namespace LoPdf\Exception {
    class Exception extends \Exception {
    }

    class LoadException extends Exception {
    }

    class SaveException extends Exception {
    }
}
