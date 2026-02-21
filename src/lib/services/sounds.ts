/**
 * Sound feedback service using Web Audio API
 * No external audio files needed — all sounds are generated programmatically.
 */

let audioCtx: AudioContext | null = null;

function getAudioContext(): AudioContext {
  if (!audioCtx || audioCtx.state === 'closed') {
    audioCtx = new AudioContext();
  }
  // Resume if suspended (browsers require user interaction first)
  if (audioCtx.state === 'suspended') {
    audioCtx.resume();
  }
  return audioCtx;
}

function playTone(
  freq: number,
  duration: number,
  type: OscillatorType = 'sine',
  volume: number = 0.15
) {
  try {
    const ctx = getAudioContext();
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();

    osc.type = type;
    osc.frequency.value = freq;
    gain.gain.value = volume;
    gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + duration);

    osc.connect(gain);
    gain.connect(ctx.destination);

    osc.start(ctx.currentTime);
    osc.stop(ctx.currentTime + duration);
  } catch {
    // Silently fail — audio is non-critical
  }
}

/**
 * Subtle beep when adding a product to the cart.
 * Short, pleasant tone at 880Hz (A5).
 */
export function playAddSound() {
  playTone(880, 0.08, 'sine', 0.12);
}

/**
 * Quick high-pitched beep for successful barcode scan.
 * Higher frequency to differentiate from manual add.
 */
export function playScanSound() {
  playTone(1200, 0.05, 'sine', 0.15);
  setTimeout(() => playTone(1500, 0.05, 'sine', 0.1), 60);
}

/**
 * Low error buzz — square wave for a "wrong" feeling.
 */
export function playErrorSound() {
  playTone(300, 0.15, 'square', 0.08);
}

/**
 * Three ascending notes for sale completion.
 * C5 → E5 → G5 (major chord arpeggio).
 */
export function playSuccessSound() {
  playTone(523, 0.15, 'sine', 0.12);  // C5
  setTimeout(() => playTone(659, 0.15, 'sine', 0.12), 120);  // E5
  setTimeout(() => playTone(784, 0.25, 'sine', 0.14), 240);  // G5 (longer)
}
