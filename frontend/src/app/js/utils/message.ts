import messages from '../../../assets/data/messages.json';

const messageElement = $('#message');
const typingSpeed = 50;

const messageQueue: { balance: number; prevBal: number, msg?: string }[] = [];
let isProcessingQueue = false;

const enqueueMessage = (balance: number, prevBal: number, msg?: string) => {
    messageQueue.push({ balance, prevBal, msg });

    if (!isProcessingQueue) {
        processMessageQueue();
    }
};

const processMessageQueue = () => {
    if (messageQueue.length === 0) {
        isProcessingQueue = false;
        return;
    }

    isProcessingQueue = true;

    const { balance, prevBal, msg } = messageQueue.shift();
    return typeMessage(balance, prevBal, msg).then(processMessageQueue);
};

const getRandomMessage = (balance: number, prevBal: number): string => {
    const textType = getTestType(balance, prevBal);
    const messagesOfType = messages[textType] as string[];
    const randomIndex = Math.floor(Math.random() * messagesOfType.length);
    return messagesOfType[randomIndex];
};

const getTestType = (balance: number, prevBal: number): string => {
    const diff = balance - prevBal;
    if (diff < -200)
        return 'negative';
    else if (diff > 200)
        return 'positive';
    else
        return 'neutral';
}

const typeMessage = (balance: number, prevBal: number, msg?: string): Promise<void> => {
    return new Promise<void>((resolve) => {
        const text = msg || getRandomMessage(balance, prevBal);
        const messageLength = text.length;
        let currentIndex = 0;
        let startTime: number;

        const typeNextCharacter = (timestamp: number) => {
            if (!startTime) {
                startTime = timestamp;
            }

            const elapsed = timestamp - startTime;
            const targetIndex = Math.floor(elapsed / typingSpeed);

            if (targetIndex > currentIndex) {
                const typedText = text.substring(0, targetIndex + 1);
                messageElement[0].textContent = typedText;
                currentIndex = targetIndex;
            }

            if (currentIndex >= messageLength - 1) {
                messageElement[0].textContent = text;
                resolve();
            } else {
                requestAnimationFrame(typeNextCharacter);
            }
        };

        requestAnimationFrame(typeNextCharacter);
    });
};


export { enqueueMessage };
