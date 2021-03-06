(ns aoc.2016.08
  (:require
   [aoc.ocr :as ocr]
   [clojure.set :as set]
   [clojure.string :as str]))

(defn parse [s]
  [(cond
     (str/starts-with? s "rect") 'rect
     (str/starts-with? s "rotate row") 'rotate-row
     (str/starts-with? s "rotate col") 'rotate-col)
   (map read-string (re-seq #"\d+" s))])

(defn rect [pixels w h]
  (set/union pixels (set (for [x (range w) y (range h)] [x y]))))

(defn rotate-row [pixels row n]
  (set (map (fn [[x y]] (if (= y row) [(mod (+ n x) 50) y] [x y])) pixels)))

(defn rotate-col [pixels col n]
  (set (map (fn [[x y]] (if (= x col) [x (mod (+ n y) 6)] [x y])) pixels)))

(defn part-* []
  (->> "input/2016/08" slurp str/split-lines (map parse)
       (reduce (fn [pixels [op args]]
                 (apply (case op
                          rect rect
                          rotate-row rotate-row
                          rotate-col rotate-col)
                        pixels
                        args))
               #{})))

(defn part-1 []
  (count (part-*)))

(defn part-2 []
  (ocr/parse (ocr/draw (part-*))))
