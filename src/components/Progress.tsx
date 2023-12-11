import { useEffect, useState } from "react";

interface ProgressProps {
    intervalDuration?: number
    totalProgressSteps?: number
    totalDuration?: number
    defaultProgress?: number
}
export default function Progress({intervalDuration = 20, totalDuration = 80, totalProgressSteps = 1, defaultProgress = 0}: ProgressProps) {
    const [progress, setProgress] = useState(defaultProgress)
    const progressStep = totalProgressSteps / (totalDuration / intervalDuration);
  

    useEffect(() => {
        const progressInterval = setInterval(() => {
            setProgress((prevProgress) => {
              const newProgress = prevProgress + progressStep;
        
              // Check if the progress has reached 100
              if (newProgress >= 100) {
                clearInterval(progressInterval); // Stop the interval
                return 100;
              }
        
              return newProgress;
            });
          }, intervalDuration);

          return () => clearInterval(progressInterval); // Stop the interval
    }, [])


    
    return (
        <div className="flex justify-center mt-3 w-full">
              <progress className="progress progress-primary w-[100%]" value={progress} max="100"></progress>
        </div>
    )
}