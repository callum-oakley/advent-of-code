(ns aoc.2017.10
  (:require
   [aoc.hash :as hash]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn rev [n knot i length]
  (reduce (fn [k j]
            (assoc k (mod (+ i j) n) (knot (mod (- (+ i length) j 1) n))))
          knot
          (range length)))

(defn sparse-hash [n lengths]
  (first (reduce (fn [[knot i skip] length]
                   [(rev n knot i length) (+ i length skip) (inc skip)])
                 [(vec (range n)) 0 0]
                 lengths)))

(defn knot-hash [s]
  (->> (concat (.getBytes s) [17 31 73 47 23]) (repeat 64) (apply concat)
       (sparse-hash 256) (partition 16) (map #(apply bit-xor %)) hash/hex))

(defn part-1 []
  (->> "input/2017/10" slurp (re-seq #"\d+") (map read-string) (sparse-hash 256)
       (take 2) (apply *)))

(defn part-2 []
  (->> "input/2017/10" slurp str/trim knot-hash))

(deftest test-examples
  (is (= [3 4 2 1 0] (sparse-hash 5 [3 4 1 5])))
  (is (= "a2582a3a0e66e6e86e3812dcb672a272" (knot-hash "")))
  (is (= "33efeb34ea91902bb2f59c9920caa6cd" (knot-hash "AoC 2017")))
  (is (= "3efbe78a8d82f29979031a4aa0b16a9d" (knot-hash "1,2,3")))
  (is (= "63960835bcdc130f0b66d7ff4f6a5a8e" (knot-hash "1,2,4"))))

(deftest test-answers
  (is (= 38628 (part-1)))
  (is (= "e1462100a34221a7f0906da15c1c979a" (part-2))))
