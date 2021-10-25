(ns aoc.2016.11
  (:require
   [aoc.search :as search]
   [clojure.math.combinatorics :as comb]
   [clojure.set :as set]
   [clojure.string :as str]
   [clojure.test :refer [deftest is]]))

(defn parse [s]
  (mapv #(->> (re-seq #"(\w+)-?\w* (microchip|generator)" %)
              (map (fn [[_ element device]] [(symbol element) (symbol device)]))
              set)
        (str/split-lines s)))

(defn safe? [floor]
  (let [microchips (set (keep (fn [[e t]] (when (= 'microchip t) e)) floor))
        generators (set (keep (fn [[e t]] (when (= 'generator t) e)) floor))]
    (or (empty? generators) (empty? (set/difference microchips generators)))))

(defn adjacent [{:keys [floors elevator] :as state}]
  (filter
   #(every? safe? (:floors %))
   (for [elevator* [(dec elevator) (inc elevator)]
         items (map set (concat (comb/combinations (seq (floors elevator)) 1)
                                (comb/combinations (seq (floors elevator)) 2)))
         :when (contains? floors elevator*)]
     (-> state
         (update-in [:floors elevator] set/difference items)
         (update-in [:floors elevator*] set/union items)
         (assoc :elevator elevator*)
         (update :steps inc)))))

(defn normalise [floors]
  (->> floors
       (map-indexed (fn [i floor]
                      (reduce (fn [acc item] (assoc-in acc item i)) {} floor)))
       (apply merge-with merge) vals frequencies))

;; Assume we can always move two objects up and one down every step
(defn heuristic [floors]
  (->> floors drop-last (map count) (reductions +)
       (map #(case % 0 0 1 1 (- (* 2 %) 3))) (apply +)))

(defn part-* [floors]
  (:steps (search/a* :steps
                     #(heuristic (:floors %))
                     {:floors floors :elevator 0 :steps 0}
                     adjacent
                     #(->> % :floors drop-last (every? empty?))
                     #(-> % (dissoc :steps) (update :floors normalise)))))

(defn part-1 []
  (-> "input/2016/11" slurp parse part-*))

(defn part-2 []
  (-> "input/2016/11" slurp parse
      (update 0 into '[[elerium generator] [elerium microchip]
                       [dilithium generator] [dilithium microchip]])
      part-*))

(def sample
  "... a hydrogen-compatible microchip and a lithium-compatible microchip.
   The second floor contains a hydrogen generator.
   The third floor contains a lithium generator.
   The fourth floor contains nothing relevant.")

(deftest test-sample
  (is (= 11 (part-* (parse sample)))))

(deftest test-answers
  (is (= 37 (part-1)))
  (is (= 61 (part-2))))
