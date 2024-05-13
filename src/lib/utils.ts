import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { cubicOut } from "svelte/easing";
import type { TransitionConfig } from "svelte/transition";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

type FlyAndScaleParams = {
	y?: number;
	x?: number;
	start?: number;
	duration?: number;
};

export const flyAndScale = (
	node: Element,
	params: FlyAndScaleParams = { y: -8, x: 0, start: 0.95, duration: 150 }
): TransitionConfig => {
	const style = getComputedStyle(node);
	const transform = style.transform === "none" ? "" : style.transform;

	const scaleConversion = (
		valueA: number,
		scaleA: [number, number],
		scaleB: [number, number]
	) => {
		const [minA, maxA] = scaleA;
		const [minB, maxB] = scaleB;

		const percentage = (valueA - minA) / (maxA - minA);
		const valueB = percentage * (maxB - minB) + minB;

		return valueB;
	};

	const styleToString = (
		style: Record<string, number | string | undefined>
	): string => {
		return Object.keys(style).reduce((str, key) => {
			if (style[key] === undefined) return str;
			return str + `${key}:${style[key]};`;
		}, "");
	};

	return {
		duration: params.duration ?? 200,
		delay: 0,
		css: (t) => {
			const y = scaleConversion(t, [0, 1], [params.y ?? 5, 0]);
			const x = scaleConversion(t, [0, 1], [params.x ?? 0, 0]);
			const scale = scaleConversion(t, [0, 1], [params.start ?? 0.95, 1]);

			return styleToString({
				transform: `${transform} translate3d(${x}px, ${y}px, 0) scale(${scale})`,
				opacity: t
			});
		},
		easing: cubicOut
	};
};

export const validateInteger = (numberString: string, value_setter: (n: number) => void, error_setter: (msg: string) => void) => {
	const number = Number.parseInt(numberString);
	if (Number.isNaN(number)) {
		error_setter('Must be an integer');
		return false;
	}

	value_setter(number);
	return true;
};

export const validateUnsigned = (numberString: string, value_setter: (n: number) => void, error_setter: (msg: string) => void) => {
	const number = Number.parseInt(numberString);
	if (Number.isNaN(number)) {
		error_setter('Must be an integer');
		return false;
	}

	if (number < 0) {
		error_setter('Must be non-negative');
		return false;
	}

	value_setter(number);
	return true;
};

export const validateUnsignedDouble = (numberString: string, value_setter: (n: number) => void, error_setter: (msg: string) => void) => {
	const number = Number.parseFloat(numberString);
	if (Number.isNaN(number)) {
		error_setter('Must be a number');
		return false;
	}

	if (number < 0) {
		error_setter('Must be non-negative');
		return false;
	}

	value_setter(number);
	return true;
}

export const validateStringLongerThan = (val: string, len: number, error_setter: (msg: string) => void) => {
	if (val.length <= len) {
		error_setter(`Must be longer than ${len} characters`);
		return false;
	}

	return true;
};

export const validateStringShorterThan = (val: string, len: number, error_setter: (msg: string) => void) => {
	if (val.length >= len) {
		error_setter(`Must be shorter than ${len} characters`);
		return false;
	}

	return true;
};