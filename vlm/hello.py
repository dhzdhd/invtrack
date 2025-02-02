from transformers import AutoProcessor, AutoModelForImageTextToText

def main():
    processor = AutoProcessor.from_pretrained("Qwen/Qwen2.5-VL-3B-Instruct")
    model = AutoModelForImageTextToText.from_pretrained("Qwen/Qwen2.5-VL-3B-Instruct")


if __name__ == "__main__":
    main()
