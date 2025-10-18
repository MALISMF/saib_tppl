import pytest

from plib import Point

@pytest.fixture
def points():
    return Point(0, 0), Point(2, 2)

class TestPoint:

    def test_creation(self):
        p = Point(1, 2)
        assert p.x == 1 and p.y == 2

        with pytest.raises(TypeError):
            Point(1.5, 1.5)

    def test_add(self, points):
        p1, p2 = points
        assert p2 + p1 == Point(2, 2)
    
    def test_sub(self, points):
        p1, p2 = points
        assert p2 - p1 == Point(2, 2)
        assert p1 - p2 == -Point(2, 2)
    
    def test_distance_to(self):
        p1 = Point(0, 0)
        p2 = Point(2, 0)
        assert p1.to(p2) == 2

    @pytest.mark.parametrize(
            "p1, p2, distance",
            [(Point(0, 0), Point(0, 10), 10),
             (Point(0, 0), Point(10, 0), 10),
             (Point(0, 0), Point(1, 1), 1.414)]
    )
    def test_distance_all_axis(self, p1, p2, distance):
        assert p1.to(p2) == pytest.approx(distance, 0.001)
    
    def test_iadd(self, points):
        p1, p2 = points
        p1 += p2
        assert p1 == Point(2, 2)
    
    def test_neg(self):
        p = Point(3, 4)
        neg_p = -p
        assert neg_p == Point(-3, -4)
    
    def test_eq_with_wrong_type(self):
        p = Point(1, 2)
        with pytest.raises(NotImplementedError):
            p == "not a point"
    
    def test_str_repr(self):
        p = Point(5, 10)
        assert str(p) == "Point(5, 10)"
        assert repr(p) == "Point(5, 10)"
    
    def test_is_center(self):
        center = Point(0, 0)
        not_center = Point(1, 0)
        assert center.is_center() is True
        assert not_center.is_center() is False
    
    def test_to_json(self):
        p = Point(3, 4)
        json_str = p.to_json()
        expected = '{"x": 3, "y": 4}'
        assert json_str == expected
    
    def test_from_json(self):
        json_str = '{"x": 5, "y": 6}'
        p = Point.from_json(json_str)
        assert p == Point(5, 6)
    
    def test_json_roundtrip(self):
        original = Point(7, 8)
        json_str = original.to_json()
        restored = Point.from_json(json_str)
        assert original == restored