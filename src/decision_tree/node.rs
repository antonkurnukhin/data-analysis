// Определение типов узлов для упрощения понимания
// Leaf - Листовой (конечный) узел
// Internal - Промежуточный узел
pub enum NodeType {
    Leaf,
    Internal,
}

// Узловой объект дерева решений. По мере роста дерева решений каждый узел может
// содержать в качестве атрибутов другие узловые объекты.
//
// Исключением является случай, когда дерево решений достигло листового узла.
//
// feature_index: usize
//       Индекс признака для разделения
// threshold: f64
//      Значение порога для разделения
// left: Node
//      Левый дочерний узел
// right: Node
//      Правый дочерний узел
pub struct Node {
    node_type: NodeType,          // Тип узла (листовой или внутренний)
    feature_index: Option<usize>, // Индекс признака для разделения
    threshold: Option<f64>,       // Значение порога для разделения
    left: Option<Box<Node>>,      // Левый дочерний узел
    right: Option<Box<Node>>,     // Правый дочерний узел
    value: Option<f64>,           // Значение для листового узла
}

impl Node {
    // Конструктор для создания нового узла
    pub fn new(node_type: NodeType) -> Self {
        Node {
            node_type,
            feature_index: None,
            threshold: None,
            left: None,
            right: None,
            value: None,
        }
    }

    // Сеттер для задания значения индекса признака для разделения
    pub fn set_feature_index(&mut self, feature_index: usize) {
        self.feature_index = Some(feature_index);
    }
    // Геттер для значения индекса признака для разделения
    pub fn get_feature_index(self) -> Option<usize> {
        self.feature_index
    }

    // Сеттер для задания значения порога для разделения
    pub fn set_threshold(&mut self, threshold: f64) {
        self.threshold = Some(threshold);
    }
    // Геттер для значения порога для разделения
    pub fn get_threshold(self) -> Option<f64> {
        self.threshold
    }

    // Сеттер для задания значения порога для разделения
    pub fn set_left(&mut self, node: Box<Node>) {
        self.left = Some(node);
    }
    // Геттер для значения порога для разделения
    pub fn get_left(self) -> Option<Box<Node>> {
        self.left
    }

    // Сеттер для задания значения порога для разделения
    pub fn set_right(&mut self, node: Box<Node>) {
        self.right = Some(node);
    }
    // Геттер для значения порога для разделения
    pub fn get_right(self) -> Option<Box<Node>> {
        self.right
    }

    // Сеттер для задания значения конечного узлы
    pub fn set_value(&mut self, value: f64) {
        self.value = Some(value);
    }
    // Геттер для значения конечного узлы
    pub fn get_value(self) -> Option<f64> {
        self.value
    }

    // Метод для получения дочерних узлов (если они существуют)
    pub fn get_children(&self) -> (Option<&Box<Node>>, Option<&Box<Node>>) {
        (self.left.as_ref(), self.right.as_ref())
    }
}
