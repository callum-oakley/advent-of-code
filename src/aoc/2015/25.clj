(ns aoc.2015.25
  (:require
   [clojure.test :refer [deftest is]]))

(defn mod-pow [b e m]
  (.modPow (biginteger b) (biginteger e) (biginteger m)))

(defn code [n]
  (mod (* 20151125 (mod-pow 252533 (dec n) 33554393)) 33554393))

(defn triangle [n]
  (quot (* n (inc n)) 2))

(defn part-1 []
  (let [[row col] (->> "input/2015/25" slurp (re-seq #"\d+") (map read-string))]
    (code (+ col (triangle (- (+ row col) 2))))))

(deftest test-answers
  (is (= 2650453 (part-1))))